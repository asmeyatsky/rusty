use crate::domain::models::LabsRegistry;
use crate::domain::services::find_project_by_name;
use crate::protocol::types::ToolCallResult;

pub fn handle(registry: &LabsRegistry, project_name: &str) -> ToolCallResult {
    match find_project_by_name(&registry.projects, project_name) {
        Some(project) => {
            let detail = serde_json::json!({
                "name": project.name,
                "description": project.description,
                "status": project.status.to_string(),
                "language": project.language,
                "tags": project.tags,
                "repo_url": project.repo_url,
                "binary_path": project.binary_path,
            });
            ToolCallResult::success(serde_json::to_string_pretty(&detail).unwrap())
        }
        None => ToolCallResult::error(format!("Project '{}' not found", project_name)),
    }
}
