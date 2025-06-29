mod file_scanner;
mod function_analyzer;
mod function_extractor;

use file_scanner::find_rust_files;
use function_analyzer::{FunctionAnalysisResult, analyze_file_functions};

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
    let files = find_rust_files(&config.search_directory);

    if files.is_empty() {
        println!(
            "No Rust files found in directory: {}",
            config.search_directory
        );
        return;
    }

    for file in files {
        analyze_and_display_file(&file);
    }
}

/// Analyzes a single file and displays the results
fn analyze_and_display_file(file_path: &str) {
    println!("{}", file_path);
    let results = analyze_file_functions(file_path);

    for result in results {
        display_function_result(&result);
    }
}

/// Displays the analysis result for a single function
fn display_function_result(result: &FunctionAnalysisResult) {
    println!(
        "  - fn {}: total={} lines, code={}, comment={}, empty={}",
        result.name, result.total, result.code, result.comment, result.empty
    );
}

fn main() {
    let config = Config::default();
    run_analysis(&config);
}
