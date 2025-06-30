mod file_scanner;
mod function_analyzer;
mod function_extractor;
mod output_formatter;

use file_scanner::find_rust_files;
use function_analyzer::analyze_all_files_sorted_by_code;
use output_formatter::OutputFormatter;

/// Configuration for the analysis
struct Config {
    pub search_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search_directory: "./src".to_string(),
        }
    }
}

/// Runs the function analysis for all Rust files in the configured directory
fn run_analysis(config: &Config) {
    let formatter = OutputFormatter::new();
    let files = find_rust_files(&config.search_directory);

    if files.is_empty() {
        formatter.display_no_files_message(&config.search_directory);
        return;
    }

    formatter.display_analysis_header(files.len());

    // Analyze all functions across all files and sort by code size
    let all_results = analyze_all_files_sorted_by_code(&files);

    formatter.display_results(&all_results);
}

fn main() {
    let config = Config::default();
    run_analysis(&config);
}
