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
use analyzer::{analyze_function_complete, extract_function_spans};
use file_scanner::find_rust_files;
use output_formatter::OutputFormatter;
use std::fs;
use std::path::Path;

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Normalizes file path separators to forward slashes for consistent output across platforms
fn normalize_path(path: &str) -> String {
    Path::new(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

// ============================================================================
/// Reads a Rust file and returns its content as a string
pub fn read_rust_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

/// Runs the function analysis for all Rust files in the configured directory
pub fn run_analysis(cli: &Client) {
    let formatter = OutputFormatter::with_format(cli.format.clone());

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

// ============================================================================
// FILE ANALYSIS FUNCTIONS
// ============================================================================

/// Analyzes all functions in a Rust file and returns analysis results
pub fn analyze_file_functions(path: &str) -> Vec<FunctionAnalysisResult> {
    let source = read_rust_file(path);
    let function_spans = extract_function_spans(&source);

    function_spans
        .iter()
        .map(|span| analyze_function_complete(span, &source))
        .collect()
}

/// Analyzes all functions across multiple files and returns unsorted results
pub fn analyze_all_files(file_paths: &[String]) -> Vec<FunctionAnalysisResult> {
    let mut all_results = Vec::new();

    for path in file_paths {
        let mut file_results = analyze_file_functions(path);
        // Add file path information to each result for context
        // Normalize path separators for consistent output across platforms
        let normalized_path = normalize_path(path);
        for result in &mut file_results {
            // We'll modify the name to include the normalized file path
            result.name = format!("{}::{}", normalized_path, result.name);
        }
        all_results.extend(file_results);
    }

    all_results
}
