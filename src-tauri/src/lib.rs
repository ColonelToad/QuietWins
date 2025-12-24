mod db;
mod tray;

use tauri::menu::{Menu, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

#[tauri::command]
fn add_win(app: tauri::AppHandle, date: String, text: String, tags: String) -> Result<(), String> {
    db::add_win(&app, &date, &text, &tags).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Build the tray menu
            let log_win = MenuItemBuilder::new("Log Win").id("log_win").build(app)?;
            let view_log = MenuItemBuilder::new("View Log").id("view_log").build(app)?;
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
            
            let menu = Menu::with_items(app, &[&log_win, &view_log, &quit])?;
            
            // Create the tray icon
            let _tray = TrayIconBuilder::new()
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
                tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, _shortcut, _event| {
                    // TODO: Show InputWindow logic here
                    println!("Shortcut triggered!");
                })
                .build()
            )?;
            
            app.global_shortcut().register(shortcut)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_win])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}