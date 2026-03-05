use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::models::{ScaffoldResult, StepResult};
use crate::domain::ports::OutputSink;

pub struct ConsoleOutput {
    multi: MultiProgress,
    bars: Mutex<HashMap<String, ProgressBar>>,
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleOutput {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
            bars: Mutex::new(HashMap::new()),
        }
    }
}

impl OutputSink for ConsoleOutput {
    fn step_start(&self, name: &str) {
        let spinner_style = ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

        let pb = self.multi.add(ProgressBar::new_spinner());
        pb.set_style(spinner_style);
        pb.set_message(format!("{} ...", name));
        pb.enable_steady_tick(std::time::Duration::from_millis(80));

        self.bars.lock().unwrap().insert(name.to_string(), pb);
    }

    fn step_done(&self, name: &str, result: &StepResult) {
        if let Some(pb) = self.bars.lock().unwrap().remove(name) {
            pb.finish_and_clear();
            if result.success {
                println!(
                    "  {} {} ({}ms)",
                    style("✓").green().bold(),
                    name,
                    result.duration_ms
                );
            } else {
                println!("  {} {} — {}", style("✗").red().bold(), name, result.output);
            }
        }
    }

    fn finish(&self, result: &ScaffoldResult) {
        println!();
        let succeeded = result.step_results.iter().filter(|r| r.success).count();
        let total = result.step_results.len();

        if succeeded == total {
            println!(
                "{} Project '{}' scaffolded successfully ({}/{} steps, {}ms)",
                style("✓").green().bold(),
                result.project_name,
                succeeded,
                total,
                result.total_duration_ms
            );
        } else {
            println!(
                "{} Project '{}' completed with errors ({}/{} steps succeeded, {}ms)",
                style("✗").red().bold(),
                result.project_name,
                succeeded,
                total,
                result.total_duration_ms
            );
        }
    }
}
