use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::config::Config;

pub enum PathMessage {
    GetPath { path: String },
    SomeError { error: String },
}

pub const FLUTTERREPO: &str = "https://github.com/flutter/flutter.git";

pub static CLONEPATH: Lazy<PathMessage> = Lazy::new(get_clone_path);

pub enum ProcessResult {
    NotFound,
    Others,
    Successed,
}

fn get_clone_path() -> PathMessage {
    let config = &*crate::config::CONFIG;

    let targetpath = if let Some(Config {
        flutter_sdk_dir: Some(path),
        ..
    }) = config
    {
        PathBuf::from(path)
    } else {
        let Ok(home) = std::env::var("HOME") else {
        return PathMessage::SomeError { error: "Cannot find home".to_string() };
    };
        PathBuf::from(&home).join(".local").join("share")
    };
    if !targetpath.exists() && std::fs::create_dir_all(&targetpath).is_err() {
        return PathMessage::SomeError {
            error: "target path cannot be created".to_string(),
        };
    }
    PathMessage::GetPath {
        path: targetpath.to_str().unwrap().to_string(),
    }
}
