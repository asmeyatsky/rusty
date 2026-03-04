use super::models::{Fingerprint, WebSignals};
use anyhow::Result;

pub struct HttpFetchResult {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub elapsed_ms: u64,
}

pub trait HttpFetcher {
    fn fetch(&self, url: &str, timeout_ms: u64) -> Result<HttpFetchResult>;
}

pub trait FingerprintRepository {
    fn all(&self) -> &[Fingerprint];
}

pub trait SignalExtractor {
    fn extract(&self, fetch_result: &HttpFetchResult) -> WebSignals;
}
