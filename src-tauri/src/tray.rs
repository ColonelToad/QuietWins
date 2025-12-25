use tauri::AppHandle;
use tauri::menu::MenuEvent;
use tauri::{WebviewWindowBuilder, WebviewUrl};

pub fn handle_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "log_win" => {
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
        }
        "view_log" => {
            let _ = WebviewWindowBuilder::new(
                app,
                "log",
                WebviewUrl::App("/LogView".into())
            )
            .title("Quiet Wins Log")
            .center()
            .resizable(true)
            .inner_size(800.0, 600.0)
            .build();
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}