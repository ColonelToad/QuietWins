use rusqlite::{Connection, Result};
use std::path::PathBuf;
use time::OffsetDateTime;

pub fn get_db_path() -> PathBuf {
    tauri::api::path::app_data_dir()
        .expect("Failed to get app data dir")
        .join("quietwins.sqlite")
}

pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
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

pub fn add_win(date: &str, text: &str, tags: &str) -> Result<()> {
    let conn = init_db()?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    conn.execute(
        "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
        (date, text, tags, now),
    )?;
    Ok(())
}

// Add more CRUD functions as needed
