use keybinds::setup_keybinds;
use state::{Rush, RushState};
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use tray::{handle_system_tray_event, system_tray};
use window::setup_window;

mod keybinds;
mod state;
mod tray;
mod window;

pub async fn init_rush() {
    let app_state: RushState = Arc::new(Mutex::new(Rush::default()));

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(system_tray())
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            setup_window(&window);
            setup_keybinds(app);

            Ok(())
        })
        .on_system_tray_event(handle_system_tray_event)
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![window::hide_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
