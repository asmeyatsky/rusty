use anyhow::Result;

use super::models::LabsRegistry;

pub trait ProjectRepository {
    fn load_registry(&self) -> impl std::future::Future<Output = Result<LabsRegistry>> + Send;
}

#[derive(Debug)]
pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub exit_code: i32,
}

pub trait ProcessRunner {
    fn run(
        &self,
        binary: &str,
        args: &[&str],
    ) -> impl std::future::Future<Output = Result<ProcessOutput>> + Send;
}
