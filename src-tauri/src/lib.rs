mod db;
mod error;
mod models;
mod ocr;
mod state;

use db::{add_todo, delete_todo, get_todos, toggle_todo};
use image::DynamicImage;
use ocr::run_ocr;
use state::AppState;
use std::sync::Mutex;
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use xcap::Monitor;

pub use error::{Error, Result};
use ort::session::Session;
use tokenizers::Tokenizer;

#[tauri::command]
async fn hide_app_window(handle: AppHandle) -> Result<()> {
    if let Some(window) = handle.get_webview_window("main") {
        window.hide()?;
    }
    Ok(())
}

async fn take_screenshot(app: &AppHandle) {
    let monitors = Monitor::all().unwrap();

    if let Some(monitor) = monitors.first() {
        let image = monitor.capture_image().unwrap();

        if let Some(window) = app.get_webview_window("overlay") {
            if window.is_visible().unwrap_or(false) {
                window.hide().unwrap();
                app.emit("reset-ocr", ()).unwrap();
            } else {
                window.show().unwrap();
                window.set_focus().unwrap();
                window.set_always_on_top(true).unwrap();
                let dynamic_image = DynamicImage::ImageRgba8(image.clone());
                let ocr_box = run_ocr(app, dynamic_image).await;
                match ocr_box {
                    Ok(bbox) => {
                        app.emit("run-ocr", bbox).unwrap();
                    }
                    Err(err) => {
                        app.emit("error", err.to_string()).unwrap();
                    }
                }
            }
        } else {
            println!("Overlay window not found!");
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("overlay") {
                window.hide()?;
            }

            let handle = app.handle().clone();
            let ctrl_shift_s = tauri_plugin_global_shortcut::Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::KeyS,
            );
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts([ctrl_shift_s.clone()])?
                    .with_handler(move |_app, shortcut, event| {
                        if shortcut == &ctrl_shift_s && event.state == ShortcutState::Pressed {
                            // 1. Clone the handle to move into the async block
                            let shortcut_handle = handle.clone();

                            // 2. Spawn the async task
                            tauri::async_runtime::spawn(async move {
                                // Now you can await your async function
                                take_screenshot(&shortcut_handle).await
                            });
                        }
                    })
                    .build(),
            )?;

            let app_handle = app.handle().clone();
            ocr::init_ort(&app_handle).expect("Failed to initialize ORT");

            // Initialize DB and OCR models asynchronously
            tauri::async_runtime::block_on(async move {
                let db_pool = db::init_db(&app_handle).await.expect("Failed to init DB");

                let resource_path = app_handle.path().resource_dir().unwrap();

                let manga_ocr_path = resource_path.join("resources").join("manga_ocr");
                let enc_model_path = manga_ocr_path.join("encoder_model.onnx");
                let dec_model_path = manga_ocr_path.join("decoder_model.onnx");
                let tokenizer_path = manga_ocr_path.join("tokenizer.json");

                let pp_ocr_path = resource_path.join("resources").join("paddle_ocr");
                let det_model_path = pp_ocr_path.join("ppocrv5-mobile-det.onnx");

                let enc_session = Session::builder()?.commit_from_file(enc_model_path)?;
                let dec_session = Session::builder()?.commit_from_file(dec_model_path)?;
                let det_session = Session::builder()?.commit_from_file(det_model_path)?;

                let tokenizer = Tokenizer::from_file(tokenizer_path)
                    .map_err(|e| Error::Tokenizer(e.to_string()))?;

                app_handle.manage(AppState {
                    db: db_pool,
                    det_session: Mutex::new(det_session),
                    enc_session: Mutex::new(enc_session),
                    dec_session: Mutex::new(dec_session),
                    tokenizer: Mutex::new(tokenizer),
                });
                Ok::<(), anyhow::Error>(())
            })
            .expect("Failed to initialize state");

            let icon_bytes = include_bytes!("../icons/lang.ico");
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show dashboard", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let icon = Image::from_bytes(icon_bytes)?;

            TrayIconBuilder::new()
                .menu(&menu)
                .icon(icon)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            hide_app_window,
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.emit("request-minimize", ());
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
