use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

use super::{store::get_rush_dir, window::show_window};

pub fn system_tray() -> SystemTray {
    let about = CustomMenuItem::new("about_rush".to_string(), "About Rush");
    let config_folder = CustomMenuItem::new("config_folder".to_string(), "Open Rush folder");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let system_tray_menu = SystemTrayMenu::new()
        .add_item(about)
        .add_item(config_folder)
        .add_item(quit);

    SystemTray::new().with_menu(system_tray_menu)
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            show_window(window);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                app.exit(0);
            }
            "config_folder" => {
                let rush_dir = get_rush_dir();
                if let Some(rush_dir) = rush_dir {
                    match open::that(rush_dir.to_str().unwrap()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error opening directory: {}", e);
                        }
                    }
                }
            }
            "about_rush" => match open::that("https://github.com/joaopugsley/rush") {
                Ok(_) => {}
                Err(e) => {
                    println!("Error opening directory: {}", e);
                }
            },
            _ => {}
        },
        _ => {}
    }
}
