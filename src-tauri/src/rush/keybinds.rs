use device_query::{DeviceQuery, DeviceState};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::{App, Manager};

use super::window::show_window;

// not the best way to do this, but it works
pub fn setup_keybinds(app: &mut App) {
    let app_handle = Arc::new(Mutex::new(app.app_handle().clone()));
    let app_handle_thread = Arc::clone(&app_handle);
    let _keypress_thread = std::thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut triggered = false;
        loop {
            let keys = device_state.get_keys();

            if keys.contains(&device_query::Keycode::LMeta)
                && keys.contains(&device_query::Keycode::F)
            {
                if !triggered {
                    let app_handle = app_handle_thread.lock().unwrap();
                    let window = app_handle.get_window("main").unwrap();
                    show_window(window);
                }
                triggered = true;
            } else {
                triggered = false;
            }

            thread::sleep(Duration::from_millis(10));
        }
    });
}
