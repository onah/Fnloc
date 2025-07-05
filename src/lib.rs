//! Fnloc - Function Line Counter for Rust
//!
//! A tool for analyzing Rust source code and counting lines of code,
//! comments, and empty lines per function.

pub mod client;
pub mod errors;
pub mod file_scanner;
pub mod function_analyzer;
pub mod function_extractor;
pub mod output_formatter;

// Re-export commonly used types for convenience
pub use client::{Client, SortBy};
pub use errors::{AnalysisError, AnalysisResult};
pub use function_analyzer::FunctionAnalysisResult;

// Internal imports for the run_analysis function
use file_scanner::find_rust_files;
use function_analyzer::analyze_all_files;
use output_formatter::OutputFormatter;

/// Runs the function analysis for all Rust files in the configured directory
pub fn run_analysis(cli: &Client) {
    let formatter = OutputFormatter::new();

    if cli.verbose {
        println!("Analyzing directory: {}", cli.directory);
        println!("Minimum lines filter: {}", cli.min_lines);
        if let Some(limit) = cli.limit {
            println!("Display limit: {}", limit);
        }
        println!("Sort by: {:?}", cli.sort);
        println!("Output format: {:?}", cli.format);
        println!();
    }

    let files = match find_rust_files(&cli.directory) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if cli.verbose {
        println!("Found {} Rust files", files.len());
    }

    formatter.display_analysis_header(files.len());

    // Analyze all functions across all files
    let all_results = analyze_all_files(&files);

    // Filter results based on minimum lines
    let filtered_results: Vec<_> = all_results
        .into_iter()
        .filter(|result| result.total >= cli.min_lines)
        .collect();

    // Apply limit if specified
    let limited_results = if let Some(limit) = cli.limit {
        filtered_results.into_iter().take(limit).collect()
    } else {
        filtered_results
    };

    // Display results sorted by specified criteria
    match cli.sort {
        SortBy::Total | SortBy::Code => {
            formatter.display_results_sorted_by_code(&limited_results);
        }
        SortBy::Comments => {
            // TODO: Implement comment-based sorting
            formatter.display_results_sorted_by_code(&limited_results);
        }
        SortBy::Name => {
            // TODO: Implement name-based sorting
            formatter.display_results_sorted_by_code(&limited_results);
        }
    }
}
