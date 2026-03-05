use serde_json;

use crate::domain::models::LabsRegistry;
use crate::protocol::types::ToolCallResult;

pub fn handle(registry: &LabsRegistry) -> ToolCallResult {
    let summary: Vec<serde_json::Value> = registry
        .projects
        .iter()
        .map(|p| {
            serde_json::json!({
                "name": p.name,
                "description": p.description,
                "status": p.status.to_string(),
                "language": p.language,
                "tags": p.tags,
            })
        })
        .collect();

    let text = serde_json::to_string_pretty(&summary).unwrap();
    ToolCallResult::success(text)
}
