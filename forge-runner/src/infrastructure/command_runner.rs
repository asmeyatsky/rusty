use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::domain::ports::{CommandExecutor, CommandOutput};

pub struct TokioCommandExecutor;

impl CommandExecutor for TokioCommandExecutor {
    async fn execute(&self, command: &str, working_dir: &str) -> Result<CommandOutput> {
        let working_path = std::path::Path::new(working_dir);
        if !working_path.exists() {
            tokio::fs::create_dir_all(working_path)
                .await
                .with_context(|| format!("Failed to create working dir: {}", working_dir))?;
        }

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(working_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn command: {}", command))?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let stdout_handle = tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            let mut output = String::new();
            while let Ok(Some(line)) = lines.next_line().await {
                if !output.is_empty() {
                    output.push('\n');
                }
                output.push_str(&line);
            }
            output
        });

        let stderr_handle = tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            let mut output = String::new();
            while let Ok(Some(line)) = lines.next_line().await {
                if !output.is_empty() {
                    output.push('\n');
                }
                output.push_str(&line);
            }
            output
        });

        let status = child.wait().await.context("Failed to wait for command")?;
        let stdout_str = stdout_handle.await.unwrap_or_default();
        let stderr_str = stderr_handle.await.unwrap_or_default();

        Ok(CommandOutput {
            stdout: stdout_str,
            stderr: stderr_str,
            success: status.success(),
        })
    }
}
