use std::{error::Error, process::Command};

use crate::constenv::{PathMessage, ProcessResult, CLONEPATH, FLUTTERREPO};

pub fn flutter_clone() -> Result<ProcessResult, Box<dyn Error>> {
    let PathMessage::GetPath { path } = &*CLONEPATH else {
        unreachable!()
    };
    match Command::new("git")
        .args(["clone", FLUTTERREPO, path])
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
    Ok(ProcessResult::Successed)
}
