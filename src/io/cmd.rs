use std::{path::PathBuf, process::Command};
use tracing::{debug, warn};
use typed_builder::TypedBuilder;

use crate::analyzers::analyze::AnalyzerError;

#[derive(Debug, Clone, TypedBuilder)]
pub struct CmdOuput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn cmd_json<T: serde::de::DeserializeOwned>(
    bin: &str,
    args: Vec<&str>,
    cwd: PathBuf,
    allow_fail: bool,
) -> Result<T, AnalyzerError> {
    let c = run_cmd(bin, args, cwd, allow_fail)?;
    let json: T = serde_json::from_str(&c.stdout).map_err(AnalyzerError::FailedToParseFromJSON)?;
    Ok(json)
}

pub fn run_cmd(
    bin: &str,
    args: Vec<&str>,
    cwd: PathBuf,
    allow_fail: bool,
) -> Result<CmdOuput, AnalyzerError> {
    let mut c = Command::new(bin);
    c.args(args);
    c.current_dir(cwd);

    debug!("attempting to exec: {:?}", c);
    let output = c.output().map_err(AnalyzerError::CommandFailed)?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let exit_code = output.status.code().unwrap_or(999);
    debug!("got exited with code: {:?}", exit_code);

    let cmd_output = CmdOuput::builder()
        .exit_code(exit_code)
        .stdout(stdout.to_string())
        .stderr(stderr.to_string())
        .build();

    if exit_code > 0 && !allow_fail {
        warn!("exec: {:?} exited with code: {:?}", c, exit_code);
    }

    Ok(cmd_output)
}
