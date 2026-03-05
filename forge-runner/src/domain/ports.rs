use anyhow::Result;
use std::collections::HashMap;

use super::models::{Manifest, ScaffoldResult, StepResult};

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[allow(async_fn_in_trait)]
pub trait ManifestLoader {
    async fn load(&self, path: &str) -> Result<Manifest>;
}

#[allow(async_fn_in_trait)]
pub trait TemplateRenderer {
    async fn render_dir(
        &self,
        source: &str,
        dest: &str,
        vars: &HashMap<String, String>,
    ) -> Result<()>;
}

#[allow(async_fn_in_trait)]
pub trait CommandExecutor {
    async fn execute(&self, command: &str, working_dir: &str) -> Result<CommandOutput>;
}

#[allow(async_fn_in_trait)]
pub trait FileSystem {
    async fn create_dir_all(&self, path: &str) -> Result<()>;
    async fn write_file(&self, path: &str, content: &str) -> Result<()>;
}

pub trait OutputSink {
    fn step_start(&self, name: &str);
    fn step_done(&self, name: &str, result: &StepResult);
    fn finish(&self, result: &ScaffoldResult);
}
