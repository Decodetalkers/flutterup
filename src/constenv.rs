use std::path::PathBuf;

use std::sync::LazyLock;

use crate::config::Config;

pub enum PathMessage {
    GetPath { path: String },
    SomeError { error: String },
}

pub const FLUTTERREPO: &str = "https://github.com/flutter/flutter.git";

pub static CLONEPATH: LazyLock<PathMessage> = LazyLock::new(get_clone_path);

pub enum ProcessResult {
    NotFound,
    Others,
    Successded,
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
            return PathMessage::SomeError {
                error: "Cannot find home".to_string(),
            };
        };
        PathBuf::from(&home)
            .join(".local")
            .join("share")
            .join("flutterup")
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
