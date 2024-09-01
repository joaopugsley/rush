use tauri::Window;
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
pub fn hide_window(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
pub fn show_window(window: tauri::Window) {
    window.move_window(Position::Center).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap()
}

pub fn setup_window(window: &Window) {
    let w = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(is_focused) = event {
            if !is_focused {
                w.hide().unwrap();
            }
        }
    })
}
