use std::{
    error::Error,
    process::{Command, Stdio},
};

use crate::{
    config::{CONFIG, Config},
    constenv::{CLONEPATH, FLUTTERREPO, PathMessage, ProcessResult},
};

pub fn flutter_clone() -> Result<ProcessResult, Box<dyn Error>> {
    let PathMessage::GetPath { path } = &*CLONEPATH else {
        unreachable!()
    };
    let branch = if let Some(Config {
        branch: Some(branch),
        ..
    }) = &*CONFIG
    {
        branch
    } else {
        "stable"
    };
    match Command::new("git")
        .args(["clone", FLUTTERREPO, "-b", branch, path])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(process) => process,
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                return Ok(ProcessResult::NotFound);
            }
            return Ok(ProcessResult::Others);
        }
    }
    .wait()?;
    Ok(ProcessResult::Successded)
}
