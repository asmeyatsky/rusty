use stacklens_cli::domain::models::*;
use stacklens_cli::domain::services::match_fingerprints;

fn make_fp(name: &str, category: TechCategory, source: SignalSource, pattern: &str) -> Fingerprint {
    Fingerprint {
        name: name.to_string(),
        category,
        signal_source: source,
        pattern: pattern.to_string(),
    }
}

#[test]
fn test_analysis_target_from_bare_domain() {
    let target = AnalysisTarget::from_domain("example.com");
    assert_eq!(target.url, "https://example.com");
    assert_eq!(target.domain, "example.com");
}

#[test]
fn test_analysis_target_from_url() {
    let target = AnalysisTarget::from_domain("https://example.com/path");
    assert_eq!(target.url, "https://example.com/path");
    assert_eq!(target.domain, "example.com");
}

#[test]
fn test_analysis_target_trims_whitespace() {
    let target = AnalysisTarget::from_domain("  example.com  ");
    assert_eq!(target.url, "https://example.com");
    assert_eq!(target.domain, "example.com");
}

#[test]
fn test_multiple_technologies_detected() {
    let signals = WebSignals {
        html_body: r#"
            <script src="/_next/static/main.js"></script>
            <script src="https://www.googletagmanager.com/gtag/js"></script>
            <link href="https://fonts.googleapis.com/css2?family=Roboto">
        "#
        .to_string(),
        headers: vec![("server".to_string(), "nginx/1.21".to_string())],
        meta_generator: None,
        cookies: Vec::new(),
    };

    let fps = vec![
        make_fp(
            "Next.js",
            TechCategory::JsFramework,
            SignalSource::HtmlBody,
            r"__NEXT_DATA__|/_next/",
        ),
        make_fp(
            "Google Analytics",
            TechCategory::Analytics,
            SignalSource::HtmlBody,
            r"gtag\(|google-analytics|googletagmanager\.com/gtag",
        ),
        make_fp(
            "Google Fonts",
            TechCategory::Fonts,
            SignalSource::HtmlBody,
            r"fonts\.googleapis\.com|fonts\.gstatic\.com",
        ),
        make_fp(
            "Nginx",
            TechCategory::Server,
            SignalSource::HttpHeader,
            r"nginx",
        ),
    ];

    let result = match_fingerprints(&signals, &fps);
    assert_eq!(result.len(), 4);

    let names: Vec<&str> = result.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"Next.js"));
    assert!(names.contains(&"Google Analytics"));
    assert!(names.contains(&"Google Fonts"));
    assert!(names.contains(&"Nginx"));
}

#[test]
fn test_sample_html_fixture() {
    let html = std::fs::read_to_string("tests/fixtures/sample.html").unwrap();

    let signals = WebSignals {
        html_body: html,
        headers: Vec::new(),
        meta_generator: Some("WordPress 6.4".to_string()),
        cookies: Vec::new(),
    };

    let fps = vec![
        make_fp(
            "Next.js",
            TechCategory::JsFramework,
            SignalSource::HtmlBody,
            r"__NEXT_DATA__|/_next/",
        ),
        make_fp(
            "React",
            TechCategory::JsFramework,
            SignalSource::HtmlBody,
            r"react[-.]|reactDOM|data-reactroot|data-reactid",
        ),
        make_fp(
            "jQuery",
            TechCategory::JsFramework,
            SignalSource::HtmlBody,
            r"jquery[.\-]|jQuery",
        ),
        make_fp(
            "WordPress",
            TechCategory::Cms,
            SignalSource::MetaGenerator,
            r"WordPress",
        ),
        make_fp(
            "Google Analytics",
            TechCategory::Analytics,
            SignalSource::HtmlBody,
            r"gtag\(|google-analytics|googletagmanager\.com/gtag",
        ),
        make_fp(
            "Google Fonts",
            TechCategory::Fonts,
            SignalSource::HtmlBody,
            r"fonts\.googleapis\.com",
        ),
    ];

    let result = match_fingerprints(&signals, &fps);
    let names: Vec<&str> = result.iter().map(|t| t.name.as_str()).collect();

    assert!(names.contains(&"Next.js"));
    assert!(names.contains(&"React"));
    assert!(names.contains(&"jQuery"));
    assert!(names.contains(&"WordPress"));
    assert!(names.contains(&"Google Analytics"));
    assert!(names.contains(&"Google Fonts"));
}
