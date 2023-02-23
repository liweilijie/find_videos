use std::path::PathBuf;
use uuid::Uuid;

pub fn uuid_v4() -> String {
    Uuid::new_v4().as_simple().to_string()
}

pub fn home_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("$HOME not found");
    PathBuf::from(home)
}

pub fn config_dir() -> PathBuf {
    let config_dir = std::env::var("FINDV_CONFIG_HOME")
        .map_or_else(|_| home_dir().join(".config"), PathBuf::from);
    config_dir.join("find-videos")
}
