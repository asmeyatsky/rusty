use clap::Parser;

/// StackLens CLI — detect technologies used by websites
#[derive(Parser, Debug)]
#[command(name = "stacklens", version, about)]
pub struct CliArgs {
    /// Domain(s) to scan (e.g. example.com)
    pub domains: Vec<String>,

    /// Output results as JSON
    #[arg(long)]
    pub json: bool,

    /// Write output to a file
    #[arg(long, short)]
    pub output: Option<String>,

    /// HTTP request timeout in milliseconds
    #[arg(long, default_value = "10000")]
    pub timeout: u64,

    /// Read domains from a file (one per line)
    #[arg(long, short)]
    pub input: Option<String>,
}
