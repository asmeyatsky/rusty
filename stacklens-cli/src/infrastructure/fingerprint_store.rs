use std::sync::OnceLock;

use crate::domain::models::Fingerprint;
use crate::domain::ports::FingerprintRepository;

const FINGERPRINTS_JSON: &str = include_str!("../../data/fingerprints.json");

static FINGERPRINTS: OnceLock<Vec<Fingerprint>> = OnceLock::new();

#[derive(Default)]
pub struct BundledFingerprintStore;

impl BundledFingerprintStore {
    pub fn new() -> Self {
        // Eagerly parse on first construction
        FINGERPRINTS.get_or_init(|| {
            serde_json::from_str(FINGERPRINTS_JSON)
                .expect("Failed to parse bundled fingerprints.json")
        });
        BundledFingerprintStore
    }
}

impl FingerprintRepository for BundledFingerprintStore {
    fn all(&self) -> &[Fingerprint] {
        FINGERPRINTS.get().expect("Fingerprints not initialized")
    }
}
