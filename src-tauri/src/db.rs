#[derive(Serialize, Clone)]
pub struct WinWithChain {
    pub id: i64,
    pub date: String,
    pub text: String,
    pub tags: String,
    pub created_at: i64,
    pub chain_id: usize,
}

/// Simple win chain detection: group wins by shared entity/keyword and sequence
pub fn get_wins_with_chains(app_handle: &tauri::AppHandle) -> Result<Vec<WinWithChain>> {
    let wins = get_wins(app_handle)?;
    // Extract chain keys (e.g., main entity or project keyword)
    let mut chains: Vec<Vec<Win>> = Vec::new();
    let mut assigned = vec![false; wins.len()];
    for (i, win) in wins.iter().enumerate() {
        if assigned[i] {
            continue;
        }
        let mut chain: Vec<Win> = vec![win.clone()];
        assigned[i] = true;
        let key = extract_chain_key(&win.text, &win.tags);
        if key.is_empty() {
            continue;
        }
        // Group other wins with same key (and within 30 days)
        for (j, other) in wins.iter().enumerate() {
            if i == j || assigned[j] {
                continue;
            }
            let other_key = extract_chain_key(&other.text, &other.tags);
            if !other_key.is_empty() && other_key == key {
                // Optionally, check date proximity
                chain.push(other.clone());
                assigned[j] = true;
            }
        }
        chains.push(chain);
    }
    // Assign chain IDs
    let mut win_with_chain = Vec::new();
    for (chain_id, chain) in chains.iter().enumerate() {
        for win in chain {
            win_with_chain.push(WinWithChain {
                id: win.id,
                date: win.date.clone(),
                text: win.text.clone(),
                tags: win.tags.clone(),
                created_at: win.created_at,
                chain_id,
            });
        }
    }
    // Add unchained wins (chain_id = usize::MAX)
    for (i, win) in wins.iter().enumerate() {
        if !assigned[i] {
            win_with_chain.push(WinWithChain {
                id: win.id,
                date: win.date.clone(),
                text: win.text.clone(),
                tags: win.tags.clone(),
                created_at: win.created_at,
                chain_id: usize::MAX,
            });
        }
    }
    Ok(win_with_chain)
}

/// Extract a chain key from text/tags (e.g., main entity, project, or noun)
fn extract_chain_key(text: &str, tags: &str) -> String {
    // Simple: use first WORK_OF_ART/PRODUCT/ORG tag, or longest word > 5 chars
    let tag_candidates: Vec<&str> = tags.split(',').map(|t| t.trim()).collect();
    for t in &tag_candidates {
        if t == &"work_of_art" || t == &"product" || t == &"org" {
            return t.to_string();
        }
    }
    // Fallback: use longest word in text
    let mut longest = "";
    for word in text.split_whitespace() {
        if word.len() > longest.len() && word.len() > 5 {
            longest = word;
        }
    }
    longest.to_string()
}
// use super::mock_data;
use crate::nlp;
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use time::OffsetDateTime;

/// Inserts mock wins for development/testing
/*
pub fn insert_mock_data(app_handle: &tauri::AppHandle) -> Result<()> {
    let db_path = get_db_path(app_handle);
    println!("[insert_mock_data] Using DB path: {}", db_path.display());
    let conn = init_db(app_handle)?;
    #[cfg(debug_assertions)]
    {
        println!("[insert_mock_data] Clearing wins table");
        conn.execute("DELETE FROM wins", [])?;
    }
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let mock_wins = mock_data::get_mock_wins();
    for win in mock_wins.iter() {
        println!(
            "[insert_mock_data] Inserting mock win: {} | {} | {}",
            win.date, win.text, win.tags
        );
        conn.execute(
            "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
            (win.date, win.text, win.tags, now),
        )?;
    }
    Ok(())
}
*/

#[derive(Serialize)]
pub struct TagGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

pub fn get_tag_graph(app_handle: &tauri::AppHandle) -> Result<TagGraph> {
    use std::collections::HashSet;
    let conn = init_db(app_handle)?;
    let mut stmt = conn.prepare("SELECT tags FROM wins")?;
    let tag_rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut tag_sets: Vec<HashSet<String>> = Vec::new();
    let mut all_tags = HashSet::new();
    for tag_str in tag_rows.flatten() {
        let cleaned = tag_str.trim_matches(|c| c == '[' || c == ']' || c == '"');
        let tags: HashSet<String> = cleaned
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !tags.is_empty() {
            all_tags.extend(tags.iter().cloned());
            tag_sets.push(tags);
        }
    }
    // Build edges: for each win, connect all pairs of tags in that win
    let mut edge_set = HashSet::new();
    for tags in &tag_sets {
        let tags_vec: Vec<&String> = tags.iter().collect();
        for i in 0..tags_vec.len() {
            for j in (i + 1)..tags_vec.len() {
                let a = tags_vec[i].clone();
                let b = tags_vec[j].clone();
                // Store edges in sorted order to avoid duplicates
                if a < b {
                    edge_set.insert((a.clone(), b.clone()));
                } else {
                    edge_set.insert((b.clone(), a.clone()));
                }
            }
        }
    }
    Ok(TagGraph {
        nodes: all_tags.into_iter().collect(),
        edges: edge_set.into_iter().collect(),
    })
}

pub fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("quietwins.sqlite")
}

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let db_path = get_db_path(app_handle);
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let mut conn = match Connection::open(&db_path) {
        Ok(c) => c,
        Err(e) => {
            println!("[init_db] Open error: {}. Attempting repair.", e);
            backup_corrupt_db(&db_path);
            Connection::open(&db_path)?
        }
    };

    if let Err(e) = ensure_db_integrity(&db_path, &conn) {
        println!("[init_db] Integrity check failed: {}. Recreating DB.", e);
        backup_corrupt_db(&db_path);
        conn = Connection::open(&db_path)?;
    }

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
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deleted_wins (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            text TEXT NOT NULL,
            tags TEXT,
            created_at INTEGER NOT NULL,
            deleted_at INTEGER NOT NULL
        )",
        [],
    )?;
    // Seed with a default win if table is empty (first run)
    let _ = seed_default_win(&conn);
    Ok(conn)
}

fn seed_default_win(conn: &Connection) -> Result<()> {
    // Check if wins table is empty
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM wins")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    if count == 0 {
        println!("[seed_default_win] DB is empty, inserting default welcome win");
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let today = OffsetDateTime::now_utc()
            .format(&time::format_description::parse("[year]-[month]-[day]").unwrap())
            .unwrap_or_else(|_| "2025-01-01".to_string());
        conn.execute(
            "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
            (
                today,
                "Welcome to Quiet Wins! Log your first win here.",
                "welcome,start",
                now,
            ),
        )?;
        println!("[seed_default_win] Default win inserted");
    }
    Ok(())
}

fn ensure_db_integrity(db_path: &PathBuf, conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA integrity_check")?;
    let status: String = stmt.query_row([], |row| row.get(0))?;
    if status.to_lowercase() != "ok" {
        println!("[init_db] integrity_check returned '{}'.", status);
        backup_corrupt_db(db_path);
        return Err(rusqlite::Error::InvalidQuery);
    }
    Ok(())
}

fn backup_corrupt_db(db_path: &PathBuf) {
    if !db_path.exists() {
        return;
    }
    let ts = OffsetDateTime::now_utc().unix_timestamp();
    let stem = db_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("quietwins");
    let backup_name = format!("{}_corrupt_{}.bak", stem, ts);
    let backup_path = db_path.with_file_name(backup_name);
    if let Err(e) = fs::rename(db_path, &backup_path) {
        println!("[init_db] Failed to back up corrupt DB: {}", e);
    } else {
        println!(
            "[init_db] Corrupt DB backed up to {}",
            backup_path.display()
        );
    }
}

pub fn add_win(app_handle: &tauri::AppHandle, date: &str, text: &str, tags: &str) -> Result<()> {
    let db_path = get_db_path(app_handle);
    println!("[add_win] Using DB path: {}", db_path.display());
    let conn = init_db(app_handle)?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let inferred_tags = infer_tags(text);
    let all_tags = if tags.trim().is_empty() {
        inferred_tags
    } else {
        format!("[{}]", [tags, &inferred_tags].join(", "))
    };
    println!(
        "[add_win] Inserting win: {} | {} | {}",
        date, text, all_tags
    );
    let res = conn.execute(
        "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
        (date, text, all_tags, now),
    );
    match res {
        Ok(_) => {
            println!("[add_win] Insert success");
            Ok(())
        }
        Err(e) => {
            println!("[add_win] Insert error: {}", e);
            Err(e)
        }
    }
}

pub fn update_win(
    app_handle: &tauri::AppHandle,
    id: i64,
    date: &str,
    text: &str,
    tags: &str,
) -> Result<()> {
    let db_path = get_db_path(app_handle);
    println!("[update_win] Using DB path: {}", db_path.display());
    let conn = init_db(app_handle)?;
    let res = conn.execute(
        "UPDATE wins SET date = ?1, text = ?2, tags = ?3 WHERE id = ?4",
        (date, text, tags, id),
    );
    match res {
        Ok(_) => {
            println!("[update_win] Update success for id {}", id);
            Ok(())
        }
        Err(e) => {
            println!("[update_win] Update error: {}", e);
            Err(e)
        }
    }
}

pub fn delete_win(app_handle: &tauri::AppHandle, id: i64) -> Result<()> {
    println!("[delete_win] Soft delete for id {}", id);
    let conn = init_db(app_handle)?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    // Move to deleted_wins instead of hard delete
    let mut stmt = conn.prepare("SELECT date, text, tags, created_at FROM wins WHERE id = ?1")?;
    let win = stmt.query_row([id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
        ))
    })?;
    let (date, text, tags, created_at) = win;
    conn.execute(
        "INSERT INTO deleted_wins (id, date, text, tags, created_at, deleted_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (id, date, text, tags, created_at, now),
    )?;
    conn.execute("DELETE FROM wins WHERE id = ?1", [id])?;
    Ok(())
}

pub fn restore_win(app_handle: &tauri::AppHandle, id: i64) -> Result<()> {
    println!("[restore_win] Restoring win id {}", id);
    let conn = init_db(app_handle)?;
    // Move back from deleted_wins to wins
    let mut stmt =
        conn.prepare("SELECT date, text, tags, created_at FROM deleted_wins WHERE id = ?1")?;
    let win = stmt.query_row([id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
        ))
    })?;
    let (date, text, tags, created_at) = win;
    conn.execute(
        "INSERT OR REPLACE INTO wins (id, date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        (id, date, text, tags, created_at),
    )?;
    conn.execute("DELETE FROM deleted_wins WHERE id = ?1", [id])?;
    Ok(())
}

pub fn get_deleted_wins(app_handle: &tauri::AppHandle) -> Result<Vec<Win>> {
    let conn = init_db(app_handle)?;
    let mut stmt =
        conn.prepare("SELECT id, date, text, tags, created_at FROM deleted_wins ORDER BY id DESC")?;
    let wins = stmt.query_map([], |row| {
        Ok(Win {
            id: row.get(0)?,
            date: row.get(1)?,
            text: row.get(2)?,
            tags: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for win in wins {
        result.push(win?);
    }
    Ok(result)
}

pub fn cleanup_old_deletions(app_handle: &tauri::AppHandle, retention_hours: i64) -> Result<()> {
    let conn = init_db(app_handle)?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let cutoff = now - (retention_hours * 3600);
    let res = conn.execute("DELETE FROM deleted_wins WHERE deleted_at < ?1", [cutoff])?;
    println!("[cleanup_old_deletions] Deleted {} old entries", res);
    Ok(())
}

fn infer_tags(text: &str) -> String {
    // Use rule-based tag suggestion first
    let dict = [
        (
            vec!["admin", "administration", "organize", "organized"],
            vec!["admin", "organization"],
        ),
        (vec!["class", "lecture", "lesson"], vec!["class", "school"]),
        (vec!["school", "campus"], vec!["school"]),
        (
            vec!["homework", "assignment", "hw"],
            vec!["homework", "school"],
        ),
        (
            vec!["study", "studied", "studying"],
            vec!["study", "school"],
        ),
        (vec!["exam", "test", "quiz"], vec!["exam", "school"]),
        (
            vec!["project", "proj", "draft", "essay", "writing"],
            vec!["project", "work", "writing"],
        ),
        (vec!["meeting", "meet"], vec!["meeting", "admin"]),
        (vec!["email", "mail"], vec!["email", "admin"]),
        (
            vec!["walk", "walking", "walked"],
            vec!["walk", "health", "casual recreation"],
        ),
        (
            vec![
                "exercise",
                "workout",
                "exercised",
                "yoga",
                "run",
                "running",
                "swim",
                "swimming",
                "pushups",
                "cycling",
                "hiit",
                "stretch",
            ],
            vec!["exercise", "health"],
        ),
        (
            vec!["read", "reading", "readed", "book"],
            vec!["read", "learning"],
        ),
        (
            vec!["cook", "cooking", "cooked", "recipe", "meal", "salad"],
            vec!["cook", "life", "food"],
        ),
        (
            vec!["clean", "cleaning", "cleaned", "bath"],
            vec!["clean", "life"],
        ),
        (
            vec!["call", "called", "calling"],
            vec!["call", "relationships"],
        ),
        (vec!["friend", "friends"], vec!["friend", "relationships"]),
        (
            vec!["family", "families", "family bonding"],
            vec!["family", "relationships", "family bonding"],
        ),
        (vec!["rest", "rested", "resting"], vec!["rest", "health"]),
        (vec!["sleep", "slept", "sleeping"], vec!["sleep", "health"]),
        (
            vec!["casual", "recreation", "relax", "relaxing", "recreation"],
            vec!["casual recreation"],
        ),
        (vec!["work", "working", "job"], vec!["work"]),
        (vec!["bonding"], vec!["family bonding"]),
    ];
    let mut tags = HashSet::new();
    let lower = text.to_lowercase();
    for (keywords, tg) in dict.iter() {
        for kw in keywords {
            if lower.contains(kw) {
                for t in tg.iter() {
                    tags.insert(t.to_string());
                }
            }
        }
    }
    // Fallback to NLP-based if rules return nothing
    if tags.is_empty() {
        tags = nlp::suggest_tags(text).into_iter().collect();
    }
    if tags.is_empty() {
        tags.insert("misc".to_string());
    }
    tags.into_iter().collect::<Vec<_>>().join(", ")
}

#[derive(Serialize, Clone)]
pub struct Win {
    pub id: i64,
    pub date: String,
    pub text: String,
    pub tags: String,
    pub created_at: i64,
}

pub fn get_wins(app_handle: &tauri::AppHandle) -> Result<Vec<Win>> {
    let db_path = get_db_path(app_handle);
    println!("[get_wins] Using DB path: {}", db_path.display());
    let conn = match init_db(app_handle) {
        Ok(c) => c,
        Err(e) => {
            println!("[get_wins] DB init error: {}", e);
            return Err(e);
        }
    };
    let mut stmt = match conn
        .prepare("SELECT id, date, text, tags, created_at FROM wins ORDER BY created_at DESC")
    {
        Ok(s) => s,
        Err(e) => {
            println!("[get_wins] Prepare error: {}", e);
            return Err(e);
        }
    };
    let wins_iter = stmt.query_map([], |row| {
        Ok(Win {
            id: row.get(0)?,
            date: row.get(1)?,
            text: row.get(2)?,
            tags: row.get(3)?,
            created_at: row.get(4)?,
        })
    });
    let mut wins = Vec::new();
    match wins_iter {
        Ok(iter) => {
            for r in iter {
                match r {
                    Ok(win) => wins.push(win),
                    Err(e) => println!("[get_wins] Row error: {}", e),
                }
            }
        }
        Err(e) => {
            println!("[get_wins] Query map error: {}", e);
            return Err(e);
        }
    }
    println!("[get_wins] Returning {} wins", wins.len());
    Ok(wins)
}
