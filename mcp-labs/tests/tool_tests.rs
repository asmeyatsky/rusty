use anyhow::Result;
use serde_json::json;

use mcp_labs::application::tool_dispatch;
use mcp_labs::domain::models::LabsRegistry;
use mcp_labs::domain::ports::{ProcessOutput, ProcessRunner};

// ── Test fixtures ──

fn fixture_registry() -> LabsRegistry {
    let toml_str = include_str!("fixtures/labs.toml");
    toml::from_str(toml_str).unwrap()
}

// ── Mock ProcessRunner ──

struct MockProcessRunner {
    stdout: String,
    success: bool,
}

impl MockProcessRunner {
    fn success(stdout: &str) -> Self {
        Self {
            stdout: stdout.to_string(),
            success: true,
        }
    }

    fn failure() -> Self {
        Self {
            stdout: String::new(),
            success: false,
        }
    }
}

impl ProcessRunner for MockProcessRunner {
    async fn run(&self, _binary: &str, _args: &[&str]) -> Result<ProcessOutput> {
        Ok(ProcessOutput {
            stdout: self.stdout.clone(),
            stderr: if self.success {
                String::new()
            } else {
                "command failed".to_string()
            },
            success: self.success,
            exit_code: if self.success { 0 } else { 1 },
        })
    }
}

// ── list_projects ──

#[tokio::test]
async fn list_projects_returns_all() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch("list_projects", &json!({}), &registry, &runner)
        .await
        .unwrap();
    assert!(!result.is_error);
    let text = &result.content[0].text;
    assert!(text.contains("alpha"));
    assert!(text.contains("beta"));
    assert!(text.contains("gamma"));
}

// ── get_project_status ──

#[tokio::test]
async fn get_project_status_found() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch(
        "get_project_status",
        &json!({"project_name": "alpha"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(!result.is_error);
    let text = &result.content[0].text;
    assert!(text.contains("alpha"));
    assert!(text.contains("complete"));
}

#[tokio::test]
async fn get_project_status_case_insensitive() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch(
        "get_project_status",
        &json!({"project_name": "BETA"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(!result.is_error);
    let text = &result.content[0].text;
    assert!(text.contains("beta"));
}

#[tokio::test]
async fn get_project_status_not_found() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch(
        "get_project_status",
        &json!({"project_name": "nonexistent"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(result.is_error);
    assert!(result.content[0].text.contains("not found"));
}

#[tokio::test]
async fn get_project_status_missing_param() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result =
        tool_dispatch::dispatch("get_project_status", &json!({}), &registry, &runner).await;
    assert!(result.is_err());
}

// ── search_projects ──

#[tokio::test]
async fn search_by_language() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch(
        "search_projects",
        &json!({"keyword": "Rust"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(!result.is_error);
    let text = &result.content[0].text;
    assert!(text.contains("alpha"));
    assert!(text.contains("gamma"));
    assert!(!text.contains("beta"));
}

#[tokio::test]
async fn search_by_tag() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch(
        "search_projects",
        &json!({"keyword": "web"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(!result.is_error);
    let text = &result.content[0].text;
    assert!(text.contains("beta"));
}

#[tokio::test]
async fn search_no_results() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch(
        "search_projects",
        &json!({"keyword": "java"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(!result.is_error);
    assert!(result.content[0].text.contains("No projects found"));
}

// ── stack_analysis ──

#[tokio::test]
async fn stack_analysis_success() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success(r#"{"technologies":["nginx"]}"#);
    let result = tool_dispatch::dispatch(
        "stack_analysis",
        &json!({"domain": "example.com"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(!result.is_error);
    assert!(result.content[0].text.contains("nginx"));
}

#[tokio::test]
async fn stack_analysis_failure() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::failure();
    let result = tool_dispatch::dispatch(
        "stack_analysis",
        &json!({"domain": "invalid.example"}),
        &registry,
        &runner,
    )
    .await
    .unwrap();
    assert!(result.is_error);
}

#[tokio::test]
async fn stack_analysis_missing_param() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch("stack_analysis", &json!({}), &registry, &runner).await;
    assert!(result.is_err());
}

// ── Unknown tool ──

#[tokio::test]
async fn unknown_tool_returns_error() {
    let registry = fixture_registry();
    let runner = MockProcessRunner::success("");
    let result = tool_dispatch::dispatch("nonexistent_tool", &json!({}), &registry, &runner).await;
    assert!(result.is_err());
}
