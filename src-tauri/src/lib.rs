// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_os::init())
//         .plugin(tauri_plugin_global_shortcut::Builder::new().build())
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
//
use chrono::Local;
use tauri::{
    menu::{Menu, MenuItem},
    path::BaseDirectory,
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

use xcap::Monitor;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 1. Setup Global Shortcut (Ctrl + Shift + S)
            let ctrl_shift_s = tauri_plugin_global_shortcut::Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::KeyS,
            );

            let app_handle = app.handle().clone();

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts([ctrl_shift_s])?
                    .with_handler(move |_app, shortcut, event| {
                        if shortcut == &ctrl_shift_s && event.state == ShortcutState::Pressed {
                            println!("Shortcut triggered! Taking screenshot...");
                            take_screenshot(&app_handle);
                        }
                    })
                    .build(),
            )?;

            // 2. Setup System Tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .menu(&menu)
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
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        // 3. Prevent App Exit on Window Close
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn take_screenshot(app: &AppHandle) {
    // Pass AppHandle to access paths
    let monitors = Monitor::all().unwrap();

    if let Some(monitor) = monitors.first() {
        let image = monitor.capture_image().unwrap();
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("screenshot_{}.png", timestamp);

        // Resolve the Picture directory path
        let file_path = app
            .path()
            .resolve(&filename, BaseDirectory::Picture)
            .unwrap();

        let _ = image.save(&file_path);
        println!("Saved to {:?}", file_path);

        // Optional: Emit an event so frontend refreshes automatically!
        let _ = app.emit("screenshot-taken", ());
    }
}
