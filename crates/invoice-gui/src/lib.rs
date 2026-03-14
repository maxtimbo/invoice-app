mod state;
mod commands;

use state::AppState;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = AppState::init().await.expect("failed to init db");
                handle.manage(state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_items,
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}
