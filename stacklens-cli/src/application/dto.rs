pub struct ScanConfig {
    pub domains: Vec<String>,
    pub timeout_ms: u64,
    pub json_output: bool,
    pub output_file: Option<String>,
}
