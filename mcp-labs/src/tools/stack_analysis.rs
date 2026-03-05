use crate::domain::ports::ProcessRunner;
use crate::protocol::types::ToolCallResult;

pub async fn handle(runner: &impl ProcessRunner, domain: &str) -> ToolCallResult {
    match runner.run("stacklens-cli", &[domain, "--json"]).await {
        Ok(output) => {
            if output.success {
                ToolCallResult::success(output.stdout)
            } else {
                let msg = if output.stderr.is_empty() {
                    format!("stacklens-cli exited with code {}", output.exit_code)
                } else {
                    output.stderr
                };
                ToolCallResult::error(msg)
            }
        }
        Err(e) => ToolCallResult::error(format!("Failed to run stacklens-cli: {}", e)),
    }
}
