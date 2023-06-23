use once_cell::sync::Lazy;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
    pub flutter_sdk_dir: Option<String>,
    pub branch: Option<String>,
}

impl Config {
    pub fn config_from_file() -> Option<Self> {
        let Ok(home) = std::env::var("HOME") else {
            return None
        };
        let config_path = std::path::Path::new(home.as_str())
            .join(".config")
            .join("flutterup")
            .join("config.toml");
        let Ok(mut file) = std::fs::OpenOptions::new()
            .read(true)
            .open(config_path)
        else {
            return None
        };
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            return None;
        };
        toml::from_str(&buf).unwrap_or(None)
    }
}

pub static CONFIG: Lazy<Option<Config>> = Lazy::new(Config::config_from_file);
