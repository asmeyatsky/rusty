use anyhow::{Context, Result};

use crate::domain::ports::FileSystem;

pub struct TokioFileSystem;

impl FileSystem for TokioFileSystem {
    async fn create_dir_all(&self, path: &str) -> Result<()> {
        tokio::fs::create_dir_all(path)
            .await
            .with_context(|| format!("Failed to create directory: {}", path))
    }

    async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create parent dir for: {}", path))?;
        }
        tokio::fs::write(path, content)
            .await
            .with_context(|| format!("Failed to write file: {}", path))
    }
}
