use anyhow::{Context, Result};

use crate::domain::models::Manifest;
use crate::domain::ports::ManifestLoader;

pub struct SerdeYamlLoader;

impl ManifestLoader for SerdeYamlLoader {
    async fn load(&self, path: &str) -> Result<Manifest> {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read manifest: {}", path))?;
        let manifest: Manifest =
            serde_yaml::from_str(&content).with_context(|| "Failed to parse YAML manifest")?;
        Ok(manifest)
    }
}
