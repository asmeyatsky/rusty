use super::models::Project;

pub fn search_projects<'a>(projects: &'a [Project], keyword: &str) -> Vec<&'a Project> {
    let keyword_lower = keyword.to_lowercase();
    projects
        .iter()
        .filter(|p| {
            p.name.to_lowercase().contains(&keyword_lower)
                || p.description.to_lowercase().contains(&keyword_lower)
                || p.language.to_lowercase().contains(&keyword_lower)
                || p.tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&keyword_lower))
        })
        .collect()
}

pub fn find_project_by_name<'a>(projects: &'a [Project], name: &str) -> Option<&'a Project> {
    let name_lower = name.to_lowercase();
    projects
        .iter()
        .find(|p| p.name.to_lowercase() == name_lower)
}

pub fn validate_registry(projects: &[Project]) -> Result<(), String> {
    if projects.is_empty() {
        return Err("Registry contains no projects".to_string());
    }
    for (i, p) in projects.iter().enumerate() {
        if p.name.is_empty() {
            return Err(format!("Project at index {} has no name", i));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::ProjectStatus;

    fn sample_projects() -> Vec<Project> {
        vec![
            Project {
                name: "stacklens-cli".to_string(),
                description: "Website tech-stack detector".to_string(),
                status: ProjectStatus::Complete,
                language: "Rust".to_string(),
                tags: vec!["cli".to_string(), "web".to_string(), "scraping".to_string()],
                repo_url: None,
                binary_path: Some("stacklens-cli".to_string()),
            },
            Project {
                name: "forge-runner".to_string(),
                description: "Project scaffolding engine".to_string(),
                status: ProjectStatus::Complete,
                language: "Rust".to_string(),
                tags: vec![
                    "cli".to_string(),
                    "scaffolding".to_string(),
                    "yaml".to_string(),
                ],
                repo_url: None,
                binary_path: Some("forge-runner".to_string()),
            },
            Project {
                name: "mcp-labs".to_string(),
                description: "MCP server for project metadata".to_string(),
                status: ProjectStatus::InProgress,
                language: "Rust".to_string(),
                tags: vec!["mcp".to_string(), "json-rpc".to_string()],
                repo_url: None,
                binary_path: None,
            },
        ]
    }

    #[test]
    fn search_by_name() {
        let projects = sample_projects();
        let results = search_projects(&projects, "stacklens");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "stacklens-cli");
    }

    #[test]
    fn search_by_description() {
        let projects = sample_projects();
        let results = search_projects(&projects, "scaffolding");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "forge-runner");
    }

    #[test]
    fn search_by_tag() {
        let projects = sample_projects();
        let results = search_projects(&projects, "cli");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn search_by_language() {
        let projects = sample_projects();
        let results = search_projects(&projects, "rust");
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn search_case_insensitive() {
        let projects = sample_projects();
        let results = search_projects(&projects, "MCP");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "mcp-labs");
    }

    #[test]
    fn search_no_results() {
        let projects = sample_projects();
        let results = search_projects(&projects, "python");
        assert!(results.is_empty());
    }

    #[test]
    fn find_by_name_exact() {
        let projects = sample_projects();
        let result = find_project_by_name(&projects, "forge-runner");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "forge-runner");
    }

    #[test]
    fn find_by_name_case_insensitive() {
        let projects = sample_projects();
        let result = find_project_by_name(&projects, "FORGE-RUNNER");
        assert!(result.is_some());
    }

    #[test]
    fn find_by_name_not_found() {
        let projects = sample_projects();
        let result = find_project_by_name(&projects, "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn validate_valid_registry() {
        let projects = sample_projects();
        assert!(validate_registry(&projects).is_ok());
    }

    #[test]
    fn validate_empty_registry() {
        let projects: Vec<Project> = vec![];
        assert!(validate_registry(&projects).is_err());
    }

    #[test]
    fn validate_nameless_project() {
        let projects = vec![Project {
            name: "".to_string(),
            description: "test".to_string(),
            status: ProjectStatus::Planning,
            language: "Rust".to_string(),
            tags: vec![],
            repo_url: None,
            binary_path: None,
        }];
        assert!(validate_registry(&projects).is_err());
    }
}
