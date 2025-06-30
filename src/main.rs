mod file_scanner;
mod function_analyzer;
mod function_extractor;
mod output_formatter;

use file_scanner::find_rust_files;
use function_analyzer::analyze_all_files;
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

    let files = match find_rust_files(&config.search_directory) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error scanning for Rust files: {}", e);
            return;
        }
    };

    if files.is_empty() {
        formatter.display_no_files_message(&config.search_directory);
        return;
    }
    formatter.display_analysis_header(files.len());

    // Analyze all functions across all files
    let all_results = analyze_all_files(&files);

    // Display results sorted by code size
    formatter.display_results_sorted_by_code(&all_results);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let search_directory = if args.len() > 1 {
        args[1].clone()
    } else {
        "./src".to_string()
    };

    let config = Config { search_directory };
    run_analysis(&config);
}
