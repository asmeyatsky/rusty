use crate::domain::models::{AnalysisResult, AnalysisTarget};
use crate::domain::ports::{FingerprintRepository, HttpFetcher, SignalExtractor};
use crate::domain::services::match_fingerprints;

use super::dto::ScanConfig;

pub struct DetectTechnologies<'a> {
    fetcher: &'a dyn HttpFetcher,
    repo: &'a dyn FingerprintRepository,
    extractor: &'a dyn SignalExtractor,
}

impl<'a> DetectTechnologies<'a> {
    pub fn new(
        fetcher: &'a dyn HttpFetcher,
        repo: &'a dyn FingerprintRepository,
        extractor: &'a dyn SignalExtractor,
    ) -> Self {
        DetectTechnologies {
            fetcher,
            repo,
            extractor,
        }
    }

    pub fn execute(&self, config: &ScanConfig) -> Vec<AnalysisResult> {
        let fingerprints = self.repo.all();
        let mut results = Vec::new();

        for domain_input in &config.domains {
            let target = AnalysisTarget::from_domain(domain_input);
            let result = match self.fetcher.fetch(&target.url, config.timeout_ms) {
                Ok(fetch_result) => {
                    eprintln!(
                        "Fetched {} (status: {}, {}ms)",
                        target.domain, fetch_result.status, fetch_result.elapsed_ms
                    );
                    let signals = self.extractor.extract(&fetch_result);
                    let technologies = match_fingerprints(&signals, fingerprints);
                    AnalysisResult {
                        domain: target.domain,
                        technologies,
                        error: None,
                    }
                }
                Err(e) => AnalysisResult {
                    domain: target.domain,
                    technologies: Vec::new(),
                    error: Some(e.to_string()),
                },
            };
            results.push(result);
        }

        results
    }
}
