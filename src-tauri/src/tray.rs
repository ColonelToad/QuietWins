use tauri::{AppHandle};
use tauri::system_tray::SystemTrayEvent;

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "log_win" => {
                    // Show input window logic here
                }
                "view_log" => {
                    // Show log view window logic here
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
