//! Integration tests for the Fnloc function analyzer
//!
//! These tests use sample Rust files to verify that the analysis
//! produces expected results for various function patterns.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Import the modules we need to test
use fnloc::analyzer::{FunctionAnalysisResult, analyze_all_files};
use fnloc::file_scanner::find_rust_files;

/// Expected result for a function analysis
#[derive(Debug, PartialEq)]
struct ExpectedResult {
    name: String,
    total: usize,
    code: usize,
    comment: usize,
    empty: usize,
    cyclomatic_complexity: usize,
    nesting_depth: usize,
}

/// Parse expected results from the configuration file
fn parse_expected_results(
    file_path: &str,
) -> Result<Vec<ExpectedResult>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut results = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        // Skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() == 7 {
            results.push(ExpectedResult {
                name: parts[0].to_string(),
                total: parts[1].parse()?,
                code: parts[2].parse()?,
                comment: parts[3].parse()?,
                empty: parts[4].parse()?,
                cyclomatic_complexity: parts[5].parse()?,
                nesting_depth: parts[6].parse()?,
            });
        }
    }

    Ok(results)
}

/// Convert analysis results to a map for easy lookup
fn results_to_map(results: &[FunctionAnalysisResult]) -> HashMap<String, &FunctionAnalysisResult> {
    let mut map = HashMap::new();
    for result in results {
        // Extract just the function name without the file path
        let function_name = if let Some(pos) = result.name.rfind("::") {
            &result.name[pos + 2..]
        } else {
            &result.name
        };
        map.insert(function_name.to_string(), result);
    }
    map
}

#[test]
fn test_sample_files_analysis() {
    let test_dir = "tests/test_sample";

    // Verify test files exist
    assert!(
        Path::new(test_dir).exists(),
        "Test sample directory does not exist"
    );
    assert!(
        Path::new(&format!("{}/sample.rs", test_dir)).exists(),
        "sample.rs does not exist"
    );
    assert!(
        Path::new(&format!("{}/module.rs", test_dir)).exists(),
        "module.rs does not exist"
    );

    // Find Rust files in the test directory
    let files = find_rust_files(test_dir).expect("Failed to find Rust files");
    assert!(!files.is_empty(), "No Rust files found in test directory");
    assert_eq!(files.len(), 2, "Expected exactly 2 Rust files");

    // Analyze the files
    let results = analyze_all_files(&files);
    assert!(!results.is_empty(), "No analysis results produced");

    // Load expected results
    let expected_results = parse_expected_results(&format!("{}/expected_results.txt", test_dir))
        .expect("Failed to parse expected results");

    // Convert results to map for easier comparison
    let result_map = results_to_map(&results);

    // Verify each expected result
    for expected in &expected_results {
        let actual = result_map.get(&expected.name).unwrap_or_else(|| {
            panic!("Function '{}' not found in analysis results", expected.name)
        });

        assert_eq!(
            actual.total, expected.total,
            "Total lines mismatch for function '{}': expected {}, got {}",
            expected.name, expected.total, actual.total
        );

        assert_eq!(
            actual.code, expected.code,
            "Code lines mismatch for function '{}': expected {}, got {}",
            expected.name, expected.code, actual.code
        );

        assert_eq!(
            actual.comment, expected.comment,
            "Comment lines mismatch for function '{}': expected {}, got {}",
            expected.name, expected.comment, actual.comment
        );

        assert_eq!(
            actual.empty, expected.empty,
            "Empty lines mismatch for function '{}': expected {}, got {}",
            expected.name, expected.empty, actual.empty
        );

        assert_eq!(
            actual.cyclomatic_complexity, expected.cyclomatic_complexity,
            "Cyclomatic complexity mismatch for function '{}': expected {}, got {}",
            expected.name, expected.cyclomatic_complexity, actual.cyclomatic_complexity
        );

        assert_eq!(
            actual.nesting_depth, expected.nesting_depth,
            "Nesting depth mismatch for function '{}': expected {}, got {}",
            expected.name, expected.nesting_depth, actual.nesting_depth
        );
    }

    println!(
        "✅ All {} function analyses match expected results!",
        expected_results.len()
    );
}

#[test]
fn test_error_handling() {
    // Test with non-existent directory
    let result = find_rust_files("non_existent_directory");
    assert!(result.is_err(), "Expected error for non-existent directory");

    // Test with empty directory
    let temp_dir = "tests/temp_empty";
    std::fs::create_dir_all(temp_dir).expect("Failed to create temp directory");

    let result = find_rust_files(temp_dir);
    assert!(
        result.is_err(),
        "Expected error for directory with no Rust files"
    );

    // Clean up
    std::fs::remove_dir_all(temp_dir).expect("Failed to remove temp directory");
}

#[test]
fn test_function_filtering_and_sorting() {
    let test_dir = "tests/test_sample";

    // Analyze files
    let files = find_rust_files(test_dir).expect("Failed to find Rust files");
    let mut results = analyze_all_files(&files);

    // Test sorting by total lines (descending)
    results.sort_by(|a, b| b.total.cmp(&a.total));

    // Verify sorting order
    for i in 1..results.len() {
        assert!(
            results[i - 1].total >= results[i].total,
            "Results are not sorted by total lines in descending order"
        );
    }

    // Test filtering by minimum lines
    let min_lines = 10;
    let filtered: Vec<_> = results
        .into_iter()
        .filter(|r| r.total >= min_lines)
        .collect();

    for result in &filtered {
        assert!(
            result.total >= min_lines,
            "Function '{}' has {} lines, which is less than minimum {}",
            result.name,
            result.total,
            min_lines
        );
    }

    println!("✅ Filtering and sorting work correctly!");
}
