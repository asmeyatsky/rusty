pub mod get_project_status;
pub mod list_projects;
pub mod search_projects;
pub mod stack_analysis;

use crate::protocol::types::ToolDescriptor;

pub fn all_tool_descriptors() -> Vec<ToolDescriptor> {
    vec![
        ToolDescriptor {
            name: "list_projects".to_string(),
            description: "List all projects in Smeyatsky Labs with their metadata".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        ToolDescriptor {
            name: "get_project_status".to_string(),
            description: "Get detailed status and metadata for a specific project by name"
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "project_name": {
                        "type": "string",
                        "description": "The name of the project to look up"
                    }
                },
                "required": ["project_name"]
            }),
        },
        ToolDescriptor {
            name: "search_projects".to_string(),
            description: "Search projects by keyword across name, description, language, and tags"
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "keyword": {
                        "type": "string",
                        "description": "The keyword to search for"
                    }
                },
                "required": ["keyword"]
            }),
        },
        ToolDescriptor {
            name: "stack_analysis".to_string(),
            description: "Analyze the technology stack of a website using stacklens-cli"
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "domain": {
                        "type": "string",
                        "description": "The domain to analyze (e.g. example.com)"
                    }
                },
                "required": ["domain"]
            }),
        },
    ]
}
