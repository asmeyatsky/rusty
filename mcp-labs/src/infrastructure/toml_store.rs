use std::sync::OnceLock;

use anyhow::Result;

use crate::domain::models::LabsRegistry;
use crate::domain::ports::ProjectRepository;

const LABS_TOML: &str = include_str!("../../data/labs.toml");

static REGISTRY: OnceLock<LabsRegistry> = OnceLock::new();

#[derive(Default)]
pub struct EmbeddedTomlStore;

impl EmbeddedTomlStore {
    pub fn new() -> Self {
        Self
    }
}

impl ProjectRepository for EmbeddedTomlStore {
    async fn load_registry(&self) -> Result<LabsRegistry> {
        let registry = REGISTRY
            .get_or_init(|| toml::from_str(LABS_TOML).expect("Failed to parse embedded labs.toml"));
        Ok(registry.clone())
    }
}
