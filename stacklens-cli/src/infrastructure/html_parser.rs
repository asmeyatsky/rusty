use scraper::{Html, Selector};

use crate::domain::models::WebSignals;
use crate::domain::ports::{HttpFetchResult, SignalExtractor};

#[derive(Default)]
pub struct ScraperSignalExtractor;

impl ScraperSignalExtractor {
    pub fn new() -> Self {
        ScraperSignalExtractor
    }
}

impl SignalExtractor for ScraperSignalExtractor {
    fn extract(&self, fetch_result: &HttpFetchResult) -> WebSignals {
        let meta_generator = extract_meta_generator(&fetch_result.body);
        let cookies = fetch_result
            .headers
            .iter()
            .filter(|(k, _)| k == "set-cookie")
            .map(|(_, v)| v.clone())
            .collect();

        WebSignals {
            html_body: fetch_result.body.clone(),
            headers: fetch_result.headers.clone(),
            meta_generator,
            cookies,
        }
    }
}

fn extract_meta_generator(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"meta[name="generator"]"#).ok()?;
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_meta_generator_wordpress() {
        let html = r#"<html><head><meta name="generator" content="WordPress 6.4"></head></html>"#;
        assert_eq!(
            extract_meta_generator(html),
            Some("WordPress 6.4".to_string())
        );
    }

    #[test]
    fn test_extract_meta_generator_missing() {
        let html = r#"<html><head><title>Test</title></head></html>"#;
        assert_eq!(extract_meta_generator(html), None);
    }

    #[test]
    fn test_extract_cookies() {
        let extractor = ScraperSignalExtractor::new();
        let result = HttpFetchResult {
            status: 200,
            headers: vec![
                ("content-type".to_string(), "text/html".to_string()),
                (
                    "set-cookie".to_string(),
                    "_ga=GA1.2.123; Path=/".to_string(),
                ),
                (
                    "set-cookie".to_string(),
                    "PHPSESSID=abc123; Path=/".to_string(),
                ),
            ],
            body: "<html></html>".to_string(),
            elapsed_ms: 100,
        };
        let signals = extractor.extract(&result);
        assert_eq!(signals.cookies.len(), 2);
    }
}
