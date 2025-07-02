//! CLI integration tests for the Fnloc tool
//! 
//! These tests verify the command-line interface behavior
//! and various options work correctly.

use std::process::Command;
use std::str;

/// Helper function to run the fnloc binary with arguments
fn run_fnloc(args: &[&str]) -> Result<(String, String, bool), Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .output()?;
    
    let stdout = str::from_utf8(&output.stdout)?.to_string();
    let stderr = str::from_utf8(&output.stderr)?.to_string();
    let success = output.status.success();
    
    Ok((stdout, stderr, success))
}

#[test]
fn test_cli_help() {
    let (stdout, _stderr, success) = run_fnloc(&["--help"])
        .expect("Failed to run fnloc --help");
    
    assert!(success, "Help command should succeed");
    assert!(stdout.contains("Analyzes Rust functions"), "Help should contain description");
    assert!(stdout.contains("--verbose"), "Help should contain verbose option");
    assert!(stdout.contains("--min-lines"), "Help should contain min-lines option");
    assert!(stdout.contains("--limit"), "Help should contain limit option");
    assert!(stdout.contains("--sort"), "Help should contain sort option");
}

#[test]
fn test_cli_version() {
    let (stdout, _stderr, success) = run_fnloc(&["--version"])
        .expect("Failed to run fnloc --version");
    
    assert!(success, "Version command should succeed");
    assert!(stdout.contains("0.1.0"), "Version should contain version number");
}

#[test]
fn test_cli_verbose_mode() {
    let (stdout, _stderr, success) = run_fnloc(&["tests/test_sample", "--verbose"])
        .expect("Failed to run fnloc with verbose");
    
    assert!(success, "Verbose command should succeed");
    assert!(stdout.contains("Analyzing directory:"), "Verbose should show directory");
    assert!(stdout.contains("Found"), "Verbose should show file count");
    assert!(stdout.contains("Sort by:"), "Verbose should show sort criteria");
}

#[test]
fn test_cli_min_lines_filter() {
    let (stdout, _stderr, success) = run_fnloc(&["tests/test_sample", "--min-lines", "10"])
        .expect("Failed to run fnloc with min-lines filter");
    
    assert!(success, "Min-lines filter should succeed");
    
    // Parse output to verify filtering
    let lines: Vec<&str> = stdout.lines().collect();
    let function_lines: Vec<&str> = lines.iter()
        .filter(|line| line.trim().starts_with("- fn"))
        .copied()
        .collect();
    
    // Verify that all displayed functions have at least 10 lines
    for line in function_lines {
        if let Some(total_part) = line.split("total=").nth(1) {
            if let Some(total_str) = total_part.split(" lines").next() {
                let total: usize = total_str.parse()
                    .expect("Failed to parse total lines");
                assert!(total >= 10, "Function should have at least 10 lines: {}", line);
            }
        }
    }
}

#[test]
fn test_cli_limit_option() {
    let (stdout, _stderr, success) = run_fnloc(&["tests/test_sample", "--limit", "2"])
        .expect("Failed to run fnloc with limit");
    
    assert!(success, "Limit option should succeed");
    
    // Count function lines in output
    let function_count = stdout.lines()
        .filter(|line| line.trim().starts_with("- fn"))
        .count();
    
    assert!(function_count <= 2, "Should display at most 2 functions, found {}", function_count);
}

#[test]
fn test_cli_error_handling() {
    // Test with non-existent directory
    let (_stdout, stderr, success) = run_fnloc(&["non_existent_directory"])
        .expect("Failed to run fnloc with invalid directory");
    
    assert!(!success, "Should fail for non-existent directory");
    // Note: Error message goes to stderr, but cargo run might capture it differently
    // So we check if either stderr contains error or the command failed
    if !stderr.contains("Error:") && !stderr.contains("error") {
        // This is acceptable as long as the command fails (non-zero exit code)
        println!("Command failed as expected (exit code indicates error)");
    }
}

#[test]
fn test_cli_default_behavior() {
    // Test default behavior (should analyze ./src)
    let (stdout, _stderr, success) = run_fnloc(&[])
        .expect("Failed to run fnloc with default args");
    
    assert!(success, "Default behavior should succeed");
    assert!(stdout.contains("Analyzing"), "Should show analysis header");
    assert!(stdout.contains("fn"), "Should show function results");
}

#[test]
fn test_cli_sort_options() {
    // Test different sort options
    let sort_options = ["total", "code", "comments", "name"];
    
    for sort_option in &sort_options {
        let (stdout, _stderr, success) = run_fnloc(&[
            "tests/test_sample", 
            "--sort", 
            sort_option,
            "--verbose"
        ]).expect(&format!("Failed to run fnloc with sort={}", sort_option));
        
        assert!(success, "Sort option '{}' should succeed", sort_option);
        assert!(stdout.contains(&format!("Sort by: {}", 
            match *sort_option {
                "total" => "Total",
                "code" => "Code", 
                "comments" => "Comments",
                "name" => "Name",
                _ => sort_option,
            }
        )), "Verbose output should show sort criteria, got: {}", stdout);
    }
}
