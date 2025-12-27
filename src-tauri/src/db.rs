use crate::models::Todo;
use crate::state::AppState;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::fs;
use tauri::{AppHandle, Manager, State};

pub async fn init_db(app_handle: &AppHandle) -> crate::Result<Pool<Sqlite>> {
    // 1. Resolve the path: AppData/com.yourapp/langcapture.db
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");

    // Ensure the directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)?;
    }

    let db_path = app_data_dir.join("langcapture.db");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    // 2. Create the file if it doesn't exist (sqlx requires this step for SQLite)
    if !std::path::Path::new(&db_path).exists() {
        std::fs::File::create(&db_path)?;
    }

    // 3. Connect
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

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
    .await?;

    Ok(pool)
}

#[tauri::command]
pub async fn get_todos(state: State<'_, AppState>) -> crate::Result<Vec<Todo>> {
    // query_as automatically maps the row to your Todo struct
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, status, created_at FROM todos ORDER BY created_at DESC",
    )
    .fetch_all(&state.db)
    .await?;

    Ok(todos)
}

#[tauri::command]
pub async fn add_todo(title: String, state: State<'_, AppState>) -> crate::Result<()> {
    sqlx::query("INSERT INTO todos (title, status) VALUES ($1, 'pending')")
        .bind(title.clone())
        .execute(&state.db)
        .await?;
    println!("Title added: {:?}", title);
    Ok(())
}

#[tauri::command]
pub async fn toggle_todo(
    id: i64,
    current_status: String,
    state: State<'_, AppState>,
) -> crate::Result<()> {
    let new_status = if current_status == "pending" {
        "done"
    } else {
        "pending"
    };

    sqlx::query("UPDATE todos SET status = $1 WHERE id = $2")
        .bind(new_status)
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_todo(id: i64, state: State<'_, AppState>) -> crate::Result<()> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(())
}
