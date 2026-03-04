use regex::Regex;
use std::collections::HashSet;

use super::models::{DetectedTech, Fingerprint, SignalSource, WebSignals};

pub fn match_fingerprints(signals: &WebSignals, fingerprints: &[Fingerprint]) -> Vec<DetectedTech> {
    let mut detected: Vec<DetectedTech> = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();

    for fp in fingerprints {
        if seen_names.contains(&fp.name) {
            continue;
        }

        let re = match Regex::new(&format!("(?i){}", &fp.pattern)) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let matched = match fp.signal_source {
            SignalSource::HtmlBody => re.is_match(&signals.html_body),
            SignalSource::HttpHeader => signals
                .headers
                .iter()
                .any(|(k, v)| re.is_match(k) || re.is_match(v)),
            SignalSource::MetaGenerator => signals
                .meta_generator
                .as_ref()
                .is_some_and(|g| re.is_match(g)),
            SignalSource::Cookie => signals.cookies.iter().any(|c| re.is_match(c)),
        };

        if matched {
            seen_names.insert(fp.name.clone());
            detected.push(DetectedTech {
                name: fp.name.clone(),
                category: fp.category.clone(),
                signal_source: fp.signal_source.clone(),
                evidence: format!("Matched pattern: {}", fp.pattern),
            });
        }
    }

    detected.sort_by(|a, b| a.category.to_string().cmp(&b.category.to_string()));
    detected
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{SignalSource, TechCategory};

    fn make_fp(
        name: &str,
        category: TechCategory,
        source: SignalSource,
        pattern: &str,
    ) -> Fingerprint {
        Fingerprint {
            name: name.to_string(),
            category,
            signal_source: source,
            pattern: pattern.to_string(),
        }
    }

    fn empty_signals() -> WebSignals {
        WebSignals {
            html_body: String::new(),
            headers: Vec::new(),
            meta_generator: None,
            cookies: Vec::new(),
        }
    }

    #[test]
    fn test_html_body_match() {
        let signals = WebSignals {
            html_body: r#"<script src="/_next/static/chunks/main.js"></script>"#.to_string(),
            ..empty_signals()
        };
        let fps = vec![make_fp(
            "Next.js",
            TechCategory::JsFramework,
            SignalSource::HtmlBody,
            r"__NEXT_DATA__|/_next/",
        )];
        let result = match_fingerprints(&signals, &fps);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Next.js");
    }

    #[test]
    fn test_header_match() {
        let signals = WebSignals {
            headers: vec![("server".to_string(), "nginx/1.21".to_string())],
            ..empty_signals()
        };
        let fps = vec![make_fp(
            "Nginx",
            TechCategory::Server,
            SignalSource::HttpHeader,
            r"nginx",
        )];
        let result = match_fingerprints(&signals, &fps);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Nginx");
    }

    #[test]
    fn test_meta_generator_match() {
        let signals = WebSignals {
            meta_generator: Some("WordPress 6.4".to_string()),
            ..empty_signals()
        };
        let fps = vec![make_fp(
            "WordPress",
            TechCategory::Cms,
            SignalSource::MetaGenerator,
            r"WordPress",
        )];
        let result = match_fingerprints(&signals, &fps);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "WordPress");
    }

    #[test]
    fn test_cookie_match() {
        let signals = WebSignals {
            cookies: vec!["_ga=GA1.2.123456789.1234567890".to_string()],
            ..empty_signals()
        };
        let fps = vec![make_fp(
            "Google Analytics",
            TechCategory::Analytics,
            SignalSource::Cookie,
            r"_ga=",
        )];
        let result = match_fingerprints(&signals, &fps);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Google Analytics");
    }

    #[test]
    fn test_no_match() {
        let signals = WebSignals {
            html_body: "<html><body>Hello</body></html>".to_string(),
            ..empty_signals()
        };
        let fps = vec![make_fp(
            "React",
            TechCategory::JsFramework,
            SignalSource::HtmlBody,
            r"react[-.]|reactDOM|data-reactroot",
        )];
        let result = match_fingerprints(&signals, &fps);
        assert!(result.is_empty());
    }

    #[test]
    fn test_dedup_by_name() {
        let signals = WebSignals {
            html_body: "jQuery loaded, also jquery.min.js".to_string(),
            ..empty_signals()
        };
        let fps = vec![
            make_fp(
                "jQuery",
                TechCategory::JsFramework,
                SignalSource::HtmlBody,
                r"jquery[.-]",
            ),
            make_fp(
                "jQuery",
                TechCategory::JsFramework,
                SignalSource::HtmlBody,
                r"jQuery",
            ),
        ];
        let result = match_fingerprints(&signals, &fps);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_case_insensitive() {
        let signals = WebSignals {
            headers: vec![("Server".to_string(), "NGINX".to_string())],
            ..empty_signals()
        };
        let fps = vec![make_fp(
            "Nginx",
            TechCategory::Server,
            SignalSource::HttpHeader,
            r"nginx",
        )];
        let result = match_fingerprints(&signals, &fps);
        assert_eq!(result.len(), 1);
    }
}
