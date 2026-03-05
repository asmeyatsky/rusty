use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Complete,
    OnHold,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "planning"),
            ProjectStatus::InProgress => write!(f, "in_progress"),
            ProjectStatus::Complete => write!(f, "complete"),
            ProjectStatus::OnHold => write!(f, "on_hold"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub language: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub repo_url: Option<String>,
    #[serde(default)]
    pub binary_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabMeta {
    pub name: String,
    pub owner: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabsRegistry {
    pub lab: LabMeta,
    #[serde(rename = "projects")]
    pub projects: Vec<Project>,
}
