use prettytable::{row, Table};

use crate::domain::models::AnalysisResult;

pub fn print_table(results: &[AnalysisResult]) {
    for result in results {
        println!("\n=== {} ===", result.domain);

        if let Some(ref err) = result.error {
            eprintln!("  Error: {}", err);
            continue;
        }

        if result.technologies.is_empty() {
            println!("  No technologies detected.");
            continue;
        }

        let mut table = Table::new();
        table.add_row(row!["Category", "Technology", "Evidence"]);

        for tech in &result.technologies {
            table.add_row(row![tech.category, tech.name, tech.signal_source]);
        }

        table.printstd();
    }
}
