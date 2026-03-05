use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "forge-runner", about = "Scaffold projects from YAML manifests")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a scaffold manifest
    Run {
        /// Path to the forge manifest YAML
        manifest: String,

        /// Print planned actions without executing
        #[arg(long)]
        dry_run: bool,

        /// Output directory (defaults to project name)
        #[arg(long)]
        output_dir: Option<String>,
    },
    /// Validate a manifest without executing
    Validate {
        /// Path to the forge manifest YAML
        manifest: String,
    },
    /// Generate a starter forge.yaml in the current directory
    Init,
}
