use crate::domain::models::LabsRegistry;
use crate::domain::services::search_projects;
use crate::protocol::types::ToolCallResult;

pub fn handle(registry: &LabsRegistry, keyword: &str) -> ToolCallResult {
    let results = search_projects(&registry.projects, keyword);

    if results.is_empty() {
        return ToolCallResult::success(format!("No projects found matching '{}'", keyword));
    }

    let summary: Vec<serde_json::Value> = results
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

    ToolCallResult::success(serde_json::to_string_pretty(&summary).unwrap())
}
