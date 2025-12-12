use base64::prelude::*;
use std::io::Cursor;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, State,
};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use xcap::Monitor;

use sqlx::{sqlite::SqlitePoolOptions, FromRow, Pool, Sqlite};
use std::fs;

// The Todo struct needs 'FromRow' to automatically map DB rows to the struct
#[derive(Debug, serde::Serialize, serde::Deserialize, FromRow)]
pub struct Todo {
    id: i64,
    title: String,
    status: String,
    created_at: String, // You can also use chrono::NaiveDateTime if you want stricter types
}

// AppState now holds the Pool directly (it's internally thread-safe)
pub struct AppState {
    db: Pool<Sqlite>,
}

async fn init_db(app_handle: &tauri::AppHandle) -> Result<Pool<Sqlite>, String> {
    // 1. Resolve the path: AppData/com.yourapp/langcapture.db
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");

    // Ensure the directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }

    let db_path = app_data_dir.join("langcapture.db");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    // 2. Create the file if it doesn't exist (sqlx requires this step for SQLite)
    if !std::path::Path::new(&db_path).exists() {
        std::fs::File::create(&db_path).map_err(|e| e.to_string())?;
    }

    // 3. Connect
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| e.to_string())?;

    // 4. Run Migration (Create Table)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(pool)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("overlay") {
                println!("Hiding overlay window!");
                let _ = window.hide();
            }

            // 1. Setup Global Shortcut (Ctrl + Shift + S)
            let ctrl_shift_s = tauri_plugin_global_shortcut::Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::KeyS,
            );

            let screenshot_app_handle = app.handle().clone();

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts([ctrl_shift_s])?
                    .with_handler(move |_app, shortcut, event| {
                        if shortcut == &ctrl_shift_s && event.state == ShortcutState::Pressed {
                            println!("Shortcut triggered! Taking screenshot...");
                            take_screenshot(&screenshot_app_handle);
                        }
                    })
                    .build(),
            )?;

            let db_app_handle = app.handle().clone();

            // Initialize DB asynchronously
            tauri::async_runtime::block_on(async move {
                let pool = init_db(&db_app_handle).await.expect("Failed to init DB");
                // Manage the state
                db_app_handle.manage(AppState { db: pool });
            });

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
        .invoke_handler(tauri::generate_handler![
            hide_app_window,
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // 1. Always prevent the native close
                api.prevent_close();

                // 2. Tell the frontend "Hey, the user tried to close the window"
                // We use emit to send a message to the Vue app
                let _ = window.emit("request-minimize", ());
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
        // Optional: Emit an event so frontend refreshes automatically!
        let mut image_data: Vec<u8> = Vec::new();
        let _ = image.write_to(
            &mut Cursor::new(&mut image_data),
            xcap::image::ImageFormat::Png,
        );
        let base64_image_string = BASE64_STANDARD.encode(image_data);
        let _ = app.emit("screenshot-taken", base64_image_string);

        if let Some(window) = app.get_webview_window("overlay") {
            if window.is_visible().unwrap_or(false) {
                // If visible -> Hide it
                let _ = window.hide();
            } else {
                // If hidden -> Show it, Focus it, and Force top
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.set_always_on_top(true);
            }
        } else {
            println!("Overlay window not found!");
        }
    }
}

#[tauri::command]
async fn hide_app_window(handle: AppHandle) -> Result<(), String> {
    if let Some(window) = handle.get_webview_window("main") {
        println!("Hiding main window!");
        let _ = window.hide();
    }
    Ok(())
}

#[tauri::command]
async fn get_todos(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    // query_as automatically maps the row to your Todo struct
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, status, created_at FROM todos ORDER BY created_at DESC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(todos)
}

#[tauri::command]
async fn add_todo(title: String, state: State<'_, AppState>) -> Result<(), String> {
    sqlx::query("INSERT INTO todos (title, status) VALUES ($1, 'pending')")
        .bind(title.clone())
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    println!("Title added: {:?}", title);
    Ok(())
}

#[tauri::command]
async fn toggle_todo(
    id: i64,
    current_status: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let new_status = if current_status == "pending" {
        "done"
    } else {
        "pending"
    };

    sqlx::query("UPDATE todos SET status = $1 WHERE id = $2")
        .bind(new_status)
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn delete_todo(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
