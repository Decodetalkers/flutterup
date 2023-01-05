use std::{error::Error, process::Command};

use crate::constenv::ProcessResult;

pub fn run_wrapper(command: &str, args: &[String]) -> Result<ProcessResult, Box<dyn Error>> {
    match Command::new(command).args(args).spawn() {
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
