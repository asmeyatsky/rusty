use std::fs;

use anyhow::{Context, Result};

use crate::domain::models::AnalysisResult;

pub fn print_json(results: &[AnalysisResult]) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    println!("{}", json);
    Ok(())
}

pub fn write_json(results: &[AnalysisResult], path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    fs::write(path, &json).with_context(|| format!("Failed to write to {}", path))?;
    eprintln!("Results written to {}", path);
    Ok(())
}
