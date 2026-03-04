mod application;
mod cli;
mod domain;
mod infrastructure;
mod output;

use std::fs;
use std::process;

use anyhow::Result;
use clap::Parser;

use application::detect::DetectTechnologies;
use application::dto::ScanConfig;
use cli::CliArgs;
use infrastructure::fingerprint_store::BundledFingerprintStore;
use infrastructure::html_parser::ScraperSignalExtractor;
use infrastructure::http_client::ReqwestHttpFetcher;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = CliArgs::parse();

    let mut domains = args.domains;

    // Read domains from input file if provided
    if let Some(ref input_path) = args.input {
        let content = fs::read_to_string(input_path)
            .map_err(|e| anyhow::anyhow!("Failed to read input file '{}': {}", input_path, e))?;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                domains.push(trimmed.to_string());
            }
        }
    }

    if domains.is_empty() {
        eprintln!("No domains specified. Use positional args or --input <file>.");
        process::exit(1);
    }

    let config = ScanConfig {
        domains,
        timeout_ms: args.timeout,
        json_output: args.json,
        output_file: args.output,
    };

    // Wire adapters
    let fetcher = ReqwestHttpFetcher::new();
    let repo = BundledFingerprintStore::new();
    let extractor = ScraperSignalExtractor::new();

    // Execute use case
    let use_case = DetectTechnologies::new(&fetcher, &repo, &extractor);
    let results = use_case.execute(&config);

    // Route output
    if config.json_output {
        output::json::print_json(&results)?;
    } else {
        output::table::print_table(&results);
    }

    if let Some(ref path) = config.output_file {
        output::json::write_json(&results, path)?;
    }

    // Exit with error if any domain failed
    let has_errors = results.iter().any(|r| r.error.is_some());
    if has_errors {
        process::exit(1);
    }

    Ok(())
}
