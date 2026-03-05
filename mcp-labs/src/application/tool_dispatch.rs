use serde_json::Value;

use crate::domain::models::LabsRegistry;
use crate::domain::ports::ProcessRunner;
use crate::protocol::errors::McpError;
use crate::protocol::types::ToolCallResult;
use crate::tools;

pub async fn dispatch(
    tool_name: &str,
    params: &Value,
    registry: &LabsRegistry,
    runner: &impl ProcessRunner,
) -> Result<ToolCallResult, McpError> {
    match tool_name {
        "list_projects" => Ok(tools::list_projects::handle(registry)),
        "get_project_status" => {
            let project_name = params
                .get("project_name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    McpError::InvalidParams("Missing required param: project_name".to_string())
                })?;
            Ok(tools::get_project_status::handle(registry, project_name))
        }
        "search_projects" => {
            let keyword = params
                .get("keyword")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    McpError::InvalidParams("Missing required param: keyword".to_string())
                })?;
            Ok(tools::search_projects::handle(registry, keyword))
        }
        "stack_analysis" => {
            let domain = params
                .get("domain")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    McpError::InvalidParams("Missing required param: domain".to_string())
                })?;
            Ok(tools::stack_analysis::handle(runner, domain).await)
        }
        _ => Err(McpError::InvalidParams(format!(
            "Unknown tool: {}",
            tool_name
        ))),
    }
}
