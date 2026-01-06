#[tauri::command]
fn get_wins_with_chains(app: tauri::AppHandle) -> Result<Vec<db::WinWithChain>, String> {
    db::get_wins_with_chains(&app).map_err(|e| e.to_string())
}
#[tauri::command]
fn suggest_tags_for_text(text: String) -> Vec<String> {
    nlp::suggest_tags(&text)
}
use chrono::{Local, NaiveTime};
use std::fs;
use std::io::{Read, Write};
use tauri::Manager;

const NOTIF_TIME_FILE: &str = "notif_time.txt";

#[tauri::command]
fn set_notif_time(app: tauri::AppHandle, notif_time: String) -> Result<(), String> {
    let path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join(NOTIF_TIME_FILE);
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(notif_time.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_notif_time(app: tauri::AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join(NOTIF_TIME_FILE);
    if !path.exists() {
        return Ok("20:00".to_string()); // default
    }
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf.trim().to_string())
}
#[tauri::command]
fn get_tag_graph(app: tauri::AppHandle) -> Result<db::TagGraph, String> {
    db::get_tag_graph(&app).map_err(|e| e.to_string())
}
mod db;
mod mock_data;
pub mod nlp;
mod tray;

use tauri::menu::{Menu, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
fn add_win(app: tauri::AppHandle, date: String, text: String, tags: String) -> Result<(), String> {
    println!(
        "[add_win command] called with date: {}, text: {}, tags: {}",
        date, text, tags
    );
    match db::add_win(&app, &date, &text, &tags) {
        Ok(res) => {
            println!("[add_win command] success");
            Ok(res)
        }
        Err(e) => {
            println!("[add_win command] error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn update_win(
    app: tauri::AppHandle,
    id: i64,
    date: String,
    text: String,
    tags: String,
) -> Result<(), String> {
    db::update_win(&app, id, &date, &text, &tags).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_win(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    db::delete_win(&app, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn restore_win(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    db::restore_win(&app, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_deleted_wins(app: tauri::AppHandle) -> Result<Vec<db::Win>, String> {
    db::get_deleted_wins(&app).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_wins(app: tauri::AppHandle) -> Result<Vec<db::Win>, String> {
    println!("[get_wins command] called");
    match db::get_wins(&app) {
        Ok(wins) => {
            println!("[get_wins command] success, returning {} wins", wins.len());
            Ok(wins)
        }
        Err(e) => {
            println!("[get_wins command] error: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let _ = db::insert_mock_data(app.handle());
                // Run NLP on mock data and print results
                crate::nlp::run_nlp_on_mock_data();
            }
            // ...existing code...
            let log_win = MenuItemBuilder::new("Log Win").id("log_win").build(app)?;
            let view_log = MenuItemBuilder::new("View Logs").id("view_log").build(app)?;
            let settings_item = MenuItemBuilder::new("Settings").id("settings").build(app)?;
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
            // ...existing code...
            let menu = Menu::with_items(app, &[&log_win, &view_log, &settings_item, &quit])?;
            // ...existing code...
            use tauri::image::Image;
            let icon_bytes = std::fs::read("icons/icon-warm-16x16.png")?;
            let img = image::load_from_memory(&icon_bytes)?.to_rgba8();
            let (width, height) = img.dimensions();
            let icon = Image::new_owned(img.into_raw(), width, height);
            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    tray::handle_tray_event(app, event);
                })
                .build(app)?;
            // ...existing code...
            use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, GlobalShortcutExt};
            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::ALT | Modifiers::SHIFT), Code::KeyW);
            // ...existing code...
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new().with_handler(move |app, _shortcut, _event| {
                    let _ = WebviewWindowBuilder::new(
                        app,
                        "input",
                        WebviewUrl::App("/InputWindow".into())
                    )
                    .title("Log a Quiet Win")
                    .always_on_top(true)
                    .center()
                    .resizable(false)
                    .inner_size(400.0, 220.0)
                    .build();
                })
                .build()
            )?;

            app.global_shortcut().register(shortcut)?;
            // ...existing code...
            // Start NLP service as a subprocess (for bundled production builds)
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let resource_dir = app_handle.path().resource_dir();
                if let Ok(resource_dir) = resource_dir {
                    let candidates = [
                        resource_dir.join("nlp_service.exe"),
                        resource_dir.join("nlp_service"),
                    ];
                    if let Some(bin) = candidates.iter().find(|p| p.exists()) {
                        match std::process::Command::new(bin).spawn() {
                            Ok(_) => println!("[nlp] Started bundled NLP service from {}", bin.display()),
                            Err(e) => println!("[nlp] Failed to start NLP service: {}", e),
                        }
                    } else {
                        println!("[nlp] Bundled NLP service binary not found in resources directory");
                    }
                }
            });
            // ...existing code...
            // Cleanup old deleted wins (every hour, 48hr retention)
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(3600));
                    let _ = db::cleanup_old_deletions(&app_handle, 48);
                }
            });
            // ...existing code...
            // Daily and every-7-days notification scheduler thread
            use std::sync::atomic::{AtomicBool, Ordering};
            use std::sync::Once;
            static SCHEDULER_STARTED: AtomicBool = AtomicBool::new(false);
            static INIT: Once = Once::new();
            let app_handle = app.handle().clone();
            INIT.call_once(|| {
                if SCHEDULER_STARTED.swap(true, Ordering::SeqCst) {
                    println!("[notification scheduler] Already started, skipping duplicate thread.");
                    return;
                }
                std::thread::spawn(move || {
                let mut last_weekly_recap: Option<chrono::DateTime<Local>> = None;
                    loop {
                    // Read notif time from file (default to 20:00)
                    let notif_time = {
                        let path = app_handle.path().app_data_dir().expect("Failed to get app data dir").join(NOTIF_TIME_FILE);
                        std::fs::read_to_string(&path).unwrap_or_else(|_| "20:00".to_string())
                    };
                    // Read notification settings from settings.json
                    let (notif_enabled, weekly_recap_enabled, notif_frequency, daily_message, weekly_message) = {
                        let path = app_handle.path().app_data_dir().expect("Failed to get app data dir").join("settings.json");
                        if let Ok(s) = std::fs::read_to_string(&path) {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                                let notif_enabled = json.get("notifEnabled").and_then(|v| v.as_bool()).unwrap_or(true);
                                let weekly_recap_enabled = json.get("weeklyRecap").and_then(|v| v.as_bool()).unwrap_or(true);
                                let notif_frequency = json.get("notifFrequency").and_then(|v| v.as_str()).unwrap_or("daily").to_string();
                                let daily_message = json.get("dailyMessage").and_then(|v| v.as_str()).unwrap_or("Don't forget to log your quiet win today!").to_string();
                                let weekly_message = json.get("weeklyMessage").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                (notif_enabled, weekly_recap_enabled, notif_frequency, daily_message, weekly_message)
                            } else { (true, true, "daily".to_string(), "Don't forget to log your quiet win today!".to_string(), "".to_string()) }
                        } else { (true, true, "daily".to_string(), "Don't forget to log your quiet win today!".to_string(), "".to_string()) }
                    };
                    let now = Local::now();
                    let today = now.date_naive();
                    let target_time = NaiveTime::parse_from_str(&notif_time, "%H:%M").unwrap_or(NaiveTime::from_hms_opt(20, 0, 0).unwrap());
                    let target_dt = today.and_time(target_time);
                    let next_dt = if now.time() < target_time {
                        target_dt
                    } else {
                        (today.succ_opt().unwrap_or(today)).and_time(target_time)
                    };
                    let duration = next_dt.and_local_timezone(Local).unwrap() - now;
                    let sleep_secs = duration.num_seconds().max(0) as u64;
                    std::thread::sleep(std::time::Duration::from_secs(sleep_secs));
                    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
                    {
                        if notif_enabled && notif_frequency != "off" {
                            // Daily notification
                            let _ = app_handle.notification()
                                .builder()
                                .title("Quiet Wins")
                                .body(&daily_message)
                                .show();
                        }
                        // Weekly recap notification (every 7 days from last recap)
                        let should_send_weekly = match last_weekly_recap {
                            None => true,
                            Some(last) => (now - last).num_days() >= 7
                        };
                        if should_send_weekly && weekly_recap_enabled && notif_enabled {
                            // Get wins from past 7 days
                            if let Ok(wins) = crate::db::get_wins(&app_handle) {
                                let week_ago = now.date_naive() - chrono::Duration::days(6);
                                let recent: Vec<_> = wins.iter().filter(|w| {
                                    if let Ok(d) = chrono::NaiveDate::parse_from_str(&w.date, "%Y-%m-%d") {
                                        d >= week_ago && d <= now.date_naive()
                                    } else { false }
                                }).collect();
                                let count = recent.len();
                                let tags: Vec<String> = recent.iter().flat_map(|w| w.tags.split(',').map(|t| t.trim().to_string())).collect();
                                let mut tag_freq = std::collections::HashMap::new();
                                for tag in tags {
                                    *tag_freq.entry(tag).or_insert(0) += 1;
                                }
                                let mut top_tags: Vec<_> = tag_freq.into_iter().collect();
                                top_tags.sort_by(|a, b| b.1.cmp(&a.1));
                                let top_tags_str = top_tags.iter().take(3).map(|(t, _)| t.clone()).collect::<Vec<_>>().join(", ");
                                let recap_body = if count == 0 {
                                    "No wins logged this week. Start a new streak!".to_string()
                                } else {
                                    format!("You logged {count} wins this week! Top tags: {top_tags_str}")
                                };
                                let body = if !weekly_message.is_empty() {
                                    format!("{}\n{}", weekly_message, recap_body)
                                } else {
                                    recap_body
                                };
                                let _ = app_handle.notification()
                                    .builder()
                                    .title("Quiet Wins Weekly Recap")
                                    .body(&body)
                                    .show();
                                last_weekly_recap = Some(now);
                            }
                        }
                    }
                    }
                });
            });
            // ...existing code...
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_win, update_win, delete_win, restore_win, get_deleted_wins, get_wins, get_tag_graph, set_notif_time, get_notif_time, suggest_tags_for_text, get_wins_with_chains])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
