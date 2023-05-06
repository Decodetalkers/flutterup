use std::{
    error::Error,
    process::{Command, Stdio},
};

use crate::constenv::ProcessResult;

pub fn run_wrapper(command: &str, args: &[String]) -> Result<ProcessResult, Box<dyn Error>> {
    match Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
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
    Ok(ProcessResult::Successed)
}
