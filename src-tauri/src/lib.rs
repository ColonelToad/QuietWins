#[tauri::command]
fn get_tag_graph(app: tauri::AppHandle) -> Result<db::TagGraph, String> {
    db::get_tag_graph(&app).map_err(|e| e.to_string())
}
mod db;
mod tray;
mod mock_data;

use tauri::menu::{Menu, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, WebviewWindowBuilder, WebviewUrl};


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
        .setup(|app| {
            // Insert mock data in dev mode
            #[cfg(debug_assertions)]
            {
                let _ = db::insert_mock_data(app.handle());
            }
            // Build the tray menu
            let log_win = MenuItemBuilder::new("Log Win").id("log_win").build(app)?;
            let view_log = MenuItemBuilder::new("View Log").id("view_log").build(app)?;
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
            
            let menu = Menu::with_items(app, &[&log_win, &view_log, &quit])?;
            
            // Create the tray icon (explicit icon for Windows)
            use tauri::image::Image;
            // Load icon PNG as bytes and decode with image crate
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
            
            // Register global shortcut
            use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, GlobalShortcutExt};
            // Use Cmd+Alt+Shift+W (SUPER | ALT | SHIFT) to avoid conflicts
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
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_win, get_wins, get_tag_graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}