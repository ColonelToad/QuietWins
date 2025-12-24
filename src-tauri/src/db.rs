use rusqlite::{Connection, Result};
use std::path::PathBuf;
use time::OffsetDateTime;
use tauri::Manager;  // Add this import

pub fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle.path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("quietwins.sqlite")
}

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let db_path = get_db_path(app_handle);
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            text TEXT NOT NULL,
            tags TEXT,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn add_win(app_handle: &tauri::AppHandle, date: &str, text: &str, tags: &str) -> Result<()> {
    let conn = init_db(app_handle)?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    conn.execute(
        "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
        (date, text, tags, now),
    )?;
    Ok(())
}