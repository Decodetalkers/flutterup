use std::path::PathBuf;

use once_cell::sync::Lazy;

pub enum PathMessage {
    GetPath { path: String },
    SomeError { error: String },
}

pub const FLUTTERREPO: &str = "https://github.com/flutter/flutter.git";

pub static CLONEPATH: Lazy<PathMessage> = Lazy::new(|| {
    let Ok(home) = std::env::var("HOME") else {
        return PathMessage::SomeError { error: "Cannot find home".to_string() };
    };
    let targetpath = PathBuf::from(&home).join(".local").join("share");
    if !targetpath.exists() && std::fs::create_dir_all(&targetpath).is_err() {
        return PathMessage::SomeError {
            error: ".local/share cannot be created".to_string(),
        };
    }
    PathMessage::GetPath {
        path: format!("{home}/.local/share/flutterup"),
    }
});

pub enum ProcessResult {
    NotFound,
    Others,
    Successed,
}


