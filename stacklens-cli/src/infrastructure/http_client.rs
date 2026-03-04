use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use reqwest::blocking::Client;

use crate::domain::ports::{HttpFetchResult, HttpFetcher};

pub struct ReqwestHttpFetcher {
    client: Client,
}

impl Default for ReqwestHttpFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ReqwestHttpFetcher {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("StackLens/0.1 (Technology Detector)")
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .expect("Failed to build HTTP client");
        ReqwestHttpFetcher { client }
    }
}

impl HttpFetcher for ReqwestHttpFetcher {
    fn fetch(&self, url: &str, timeout_ms: u64) -> Result<HttpFetchResult> {
        let start = Instant::now();
        let response = self
            .client
            .get(url)
            .timeout(Duration::from_millis(timeout_ms))
            .send()
            .with_context(|| format!("Failed to fetch {}", url))?;

        let status = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_lowercase(),
                    v.to_str().unwrap_or("").to_string(),
                )
            })
            .collect();

        let body = response
            .text()
            .with_context(|| format!("Failed to read response body from {}", url))?;

        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok(HttpFetchResult {
            status,
            headers,
            body,
            elapsed_ms,
        })
    }
}
