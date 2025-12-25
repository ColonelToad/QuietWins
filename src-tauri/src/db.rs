use super::mock_data;
use serde::Serialize;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use time::OffsetDateTime;
use tauri::Manager;

/// Inserts mock wins for development/testing
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
        println!("[insert_mock_data] Inserting mock win: {} | {} | {}", win.date, win.text, win.tags);
        conn.execute(
            "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
            (win.date, win.text, win.tags, now),
        )?;
    }
    Ok(())
}

#[derive(Serialize)]
pub struct TagGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

pub fn get_tag_graph(app_handle: &tauri::AppHandle) -> Result<TagGraph> {
    use std::collections::{HashSet, HashMap};
    let conn = init_db(app_handle)?;
    let mut stmt = conn.prepare("SELECT tags FROM wins")?;
    let tag_rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut tag_sets: Vec<HashSet<String>> = Vec::new();
    let mut all_tags = HashSet::new();
    for row in tag_rows {
        if let Ok(tag_str) = row {
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
    }
    // Build edges: for each win, connect all pairs of tags in that win
    let mut edge_set = HashSet::new();
    for tags in &tag_sets {
        let tags_vec: Vec<&String> = tags.iter().collect();
        for i in 0..tags_vec.len() {
            for j in (i+1)..tags_vec.len() {
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
    app_handle.path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("quietwins.sqlite")
}

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let db_path = get_db_path(app_handle);
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
    println!("[add_win] Inserting win: {} | {} | {}", date, text, all_tags);
    let res = conn.execute(
        "INSERT INTO wins (date, text, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
        (date, text, all_tags, now),
    );
    match res {
        Ok(_) => {
            println!("[add_win] Insert success");
            Ok(())
        },
        Err(e) => {
            println!("[add_win] Insert error: {}", e);
            Err(e)
        }
    }
}

fn infer_tags(text: &str) -> String {
    use std::collections::HashSet;
    let dict = [
        (vec!["admin", "administration", "organize", "organized"], vec!["admin", "organization"]),
        (vec!["class", "lecture", "lesson"], vec!["class", "school"]),
        (vec!["school", "campus"], vec!["school"]),
        (vec!["homework", "assignment", "hw"], vec!["homework", "school"]),
        (vec!["study", "studied", "studying"], vec!["study", "school"]),
        (vec!["exam", "test", "quiz"], vec!["exam", "school"]),
        (vec!["project", "proj"], vec!["project", "school"]),
        (vec!["meeting", "meet"], vec!["meeting", "admin"]),
        (vec!["email", "mail"], vec!["email", "admin"]),
        (vec!["walk", "walking", "walked"], vec!["walk", "health"]),
        (vec!["exercise", "workout", "exercised"], vec!["exercise", "health"]),
        (vec!["read", "reading", "readed"], vec!["read", "learning"]),
        (vec!["cook", "cooking", "cooked"], vec!["cook", "life"]),
        (vec!["clean", "cleaning", "cleaned"], vec!["clean", "life"]),
        (vec!["call", "called", "calling"], vec!["call", "relationships"]),
        (vec!["friend", "friends"], vec!["friend", "relationships"]),
        (vec!["family", "families"], vec!["family", "relationships"]),
        (vec!["rest", "rested", "resting"], vec!["rest", "health"]),
        (vec!["sleep", "slept", "sleeping"], vec!["sleep", "health"]),
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
    if tags.is_empty() {
        tags.insert("misc".to_string());
    }
    tags.into_iter().collect::<Vec<_>>().join(", ")
}

#[derive(Serialize)]
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
    let mut stmt = match conn.prepare("SELECT id, date, text, tags, created_at FROM wins ORDER BY created_at DESC") {
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
        },
        Err(e) => {
            println!("[get_wins] Query map error: {}", e);
            return Err(e);
        }
    }
    println!("[get_wins] Returning {} wins", wins.len());
    Ok(wins)
}
