//! Fnloc - Function Line Counter for Rust
//!
//! A tool for analyzing Rust source code and counting lines of code,
//! comments, and empty lines per function.

pub mod analyzer;
pub mod client;
pub mod errors;
pub mod file_scanner;
pub mod output_formatter;

// Re-export commonly used types for convenience
pub use analyzer::FunctionAnalysisResult;
pub use client::{Client, OutputFormat};
pub use errors::{AnalysisError, AnalysisResult};

// Internal imports for the run_analysis function
use analyzer::analyze_all_files;
use file_scanner::find_rust_files;
use output_formatter::OutputFormatter;

/// Runs the function analysis for all Rust files in the configured directory
pub fn run_analysis(cli: &Client) {
    let formatter = OutputFormatter::new();

    let files = match find_rust_files(&cli.directory) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    formatter.display_analysis_header(files.len());

    // Analyze all functions across all files
    let all_results = analyze_all_files(&files);

    // Display results (sorted by code lines descending - default behavior)
    formatter.display_results_sorted_by_code(&all_results);
}
