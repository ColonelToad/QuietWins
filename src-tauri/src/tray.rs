use tauri::menu::MenuEvent;
use tauri::AppHandle;
use tauri::{WebviewUrl, WebviewWindowBuilder};

pub fn handle_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "log_win" => {
            let _ = WebviewWindowBuilder::new(app, "input", WebviewUrl::App("/InputWindow".into()))
                .title("Log a Quiet Win")
                .always_on_top(true)
                .center()
                .resizable(false)
                .inner_size(400.0, 220.0)
                .build();
        }
        "view_log" => {
            let _ = WebviewWindowBuilder::new(app, "log", WebviewUrl::App("/LogView".into()))
                .title("Quiet Wins Log")
                .center()
                .resizable(true)
                .inner_size(800.0, 600.0)
                .build();
        }
        "settings" => {
            let _ = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("/Settings".into()))
                .title("Settings")
                .center()
                .resizable(true)
                .inner_size(720.0, 540.0)
                .build();
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}
