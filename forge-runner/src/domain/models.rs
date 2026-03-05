use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    CreateDir,
    Template,
    Command,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    pub name: String,
    pub step_type: StepType,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub dest: Option<String>,
    #[serde(default)]
    pub run: Option<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub name: String,
    pub stack: String,
    #[serde(default)]
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub project: Project,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone)]
pub struct StepResult {
    pub step_name: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u128,
}

#[derive(Debug)]
pub struct ScaffoldResult {
    pub project_name: String,
    pub step_results: Vec<StepResult>,
    pub total_duration_ms: u128,
}
