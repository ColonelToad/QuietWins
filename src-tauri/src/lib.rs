// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;

mod tray;
use tauri::system_tray::{SystemTray, SystemTrayEvent};
use tauri_plugin_global_shortcut::init as global_shortcut_init;

#[tauri::command]
fn add_win(date: String, text: String, tags: String) -> Result<(), String> {
    db::add_win(&date, &text, &tags).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .system_tray(SystemTray::new())
        .plugin(global_shortcut_init())
        .invoke_handler(tauri::generate_handler![add_win])
        .setup(|app| {
            let shortcut = app.plugin::<tauri_plugin_global_shortcut::GlobalShortcut>().unwrap();
            shortcut.register("Cmd+Shift+W", move || {
                // TODO: Show InputWindow logic here
            })?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
