#[tauri::command]
fn suggest_tags_for_text(text: String) -> Vec<String> {
    nlp::suggest_tags(&text)
}
use std::fs;
use std::io::{Read, Write};
use chrono::{Local, NaiveTime, Timelike};
use tauri::Manager;

const NOTIF_TIME_FILE: &str = "notif_time.txt";

#[tauri::command]
fn set_notif_time(app: tauri::AppHandle, notif_time: String) -> Result<(), String> {
    let path = app.path().app_data_dir().expect("Failed to get app data dir").join(NOTIF_TIME_FILE);
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(notif_time.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_notif_time(app: tauri::AppHandle) -> Result<String, String> {
    let path = app.path().app_data_dir().expect("Failed to get app data dir").join(NOTIF_TIME_FILE);
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
mod tray;
mod mock_data;
pub mod nlp;

use tauri::menu::{Menu, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{WebviewWindowBuilder, WebviewUrl};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
fn add_win(app: tauri::AppHandle, date: String, text: String, tags: String) -> Result<(), String> {
    println!("[add_win command] called with date: {}, text: {}, tags: {}", date, text, tags);
    match db::add_win(&app, &date, &text, &tags) {
        Ok(res) => {
            println!("[add_win command] success");
            Ok(res)
        },
        Err(e) => {
            println!("[add_win command] error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn get_wins(app: tauri::AppHandle) -> Result<Vec<db::Win>, String> {
    println!("[get_wins command] called");
    match db::get_wins(&app) {
        Ok(wins) => {
            println!("[get_wins command] success, returning {} wins", wins.len());
            Ok(wins)
        },
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
            
            let log_win = MenuItemBuilder::new("Log Win").id("log_win").build(app)?;
            let view_log = MenuItemBuilder::new("View Log").id("view_log").build(app)?;
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
            
            let menu = Menu::with_items(app, &[&log_win, &view_log, &quit])?;
            
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
            
            use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, GlobalShortcutExt};
            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::ALT | Modifiers::SHIFT), Code::KeyW);
            
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
            
            // Daily notification scheduler thread
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                loop {
                    // Read notif time from file (default to 20:00)
                    let notif_time = {
                        let path = app_handle.path().app_data_dir().expect("Failed to get app data dir").join(NOTIF_TIME_FILE);
                        std::fs::read_to_string(&path).unwrap_or_else(|_| "20:00".to_string())
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
                        let _ = app_handle.notification()
                            .builder()
                            .title("Quiet Wins")
                            .body("Don't forget to log your quiet win today!")
                            .show();
                    }
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_win, get_wins, get_tag_graph, set_notif_time, get_notif_time, suggest_tags_for_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}