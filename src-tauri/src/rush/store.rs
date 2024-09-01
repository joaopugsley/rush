use std::{env, fs, path::PathBuf};

pub fn get_rush_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        env::var("APPDATA")
            .ok()
            .map(|app_data| PathBuf::from(app_data).join("Rush"))
    } else if cfg!(target_os = "macos") {
        env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join("Library/Application Support/Rush"))
    } else {
        env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join(".rush"))
    }
}

fn rush_dir_exists() -> bool {
    let rush_dir = get_rush_dir();
    fs::metadata(rush_dir.unwrap()).is_ok()
}

fn create_rush_dir() -> Result<(), std::io::Error> {
    let rush_dir = get_rush_dir();
    fs::create_dir_all(&rush_dir.unwrap())
}

pub fn ensure_rush_dir_exists() {
    if !rush_dir_exists() {
        create_rush_dir().unwrap();
    }
}
