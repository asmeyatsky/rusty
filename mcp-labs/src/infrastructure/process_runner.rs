use anyhow::Result;
use tokio::process::Command;

use crate::domain::ports::{ProcessOutput, ProcessRunner};

#[derive(Default)]
pub struct TokioProcessRunner;

impl TokioProcessRunner {
    pub fn new() -> Self {
        Self
    }
}

impl ProcessRunner for TokioProcessRunner {
    async fn run(&self, binary: &str, args: &[&str]) -> Result<ProcessOutput> {
        let output = Command::new(binary).args(args).output().await?;

        Ok(ProcessOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
}
