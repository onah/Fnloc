mod file_scanner;
mod function_analyzer;
mod function_extractor;

use file_scanner::find_rust_files;
use function_analyzer::{FunctionAnalysisResult, analyze_all_files_sorted_by_code};

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

    println!("Analyzing {} Rust files...\n", files.len());

    // Analyze all functions across all files and sort by code size
    let all_results = analyze_all_files_sorted_by_code(&files);

    for result in all_results {
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
