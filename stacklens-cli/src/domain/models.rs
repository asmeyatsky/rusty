use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TechCategory {
    JsFramework,
    CssFramework,
    Analytics,
    Cdn,
    Cms,
    Server,
    Waf,
    CloudProvider,
    Payment,
    Ecommerce,
    Fonts,
    ThirdParty,
    Monitoring,
    Auth,
    Maps,
    Video,
    Communication,
    Consent,
}

impl std::fmt::Display for TechCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TechCategory::JsFramework => "JS Framework",
            TechCategory::CssFramework => "CSS Framework",
            TechCategory::Analytics => "Analytics",
            TechCategory::Cdn => "CDN",
            TechCategory::Cms => "CMS",
            TechCategory::Server => "Server",
            TechCategory::Waf => "WAF",
            TechCategory::CloudProvider => "Cloud Provider",
            TechCategory::Payment => "Payment",
            TechCategory::Ecommerce => "E-commerce",
            TechCategory::Fonts => "Fonts",
            TechCategory::ThirdParty => "Third Party",
            TechCategory::Monitoring => "Monitoring",
            TechCategory::Auth => "Auth",
            TechCategory::Maps => "Maps",
            TechCategory::Video => "Video",
            TechCategory::Communication => "Communication",
            TechCategory::Consent => "Consent",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    HtmlBody,
    HttpHeader,
    MetaGenerator,
    Cookie,
}

impl std::fmt::Display for SignalSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SignalSource::HtmlBody => "HTML Body",
            SignalSource::HttpHeader => "HTTP Header",
            SignalSource::MetaGenerator => "Meta Generator",
            SignalSource::Cookie => "Cookie",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub name: String,
    pub category: TechCategory,
    pub signal_source: SignalSource,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DetectedTech {
    pub name: String,
    pub category: TechCategory,
    pub signal_source: SignalSource,
    pub evidence: String,
}

#[derive(Debug, Clone)]
pub struct WebSignals {
    pub html_body: String,
    pub headers: Vec<(String, String)>,
    pub meta_generator: Option<String>,
    pub cookies: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AnalysisTarget {
    pub original_input: String,
    pub url: String,
    pub domain: String,
}

impl AnalysisTarget {
    pub fn from_domain(input: &str) -> Self {
        let trimmed = input.trim();
        let url = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            trimmed.to_string()
        } else {
            format!("https://{}", trimmed)
        };
        let domain = url
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .split('/')
            .next()
            .unwrap_or(trimmed)
            .to_string();
        AnalysisTarget {
            original_input: trimmed.to_string(),
            url,
            domain,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    pub domain: String,
    pub technologies: Vec<DetectedTech>,
    pub error: Option<String>,
}
