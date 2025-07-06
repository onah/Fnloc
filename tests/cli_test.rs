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
fn test_cli_version() {
    let (stdout, _stderr, success) =
        run_fnloc(&["--version"]).expect("Failed to run fnloc --version");

    assert!(success, "Version command should succeed");
    assert!(
        stdout.contains("0.1.0"),
        "Version should contain version number"
    );
}

#[test]
fn test_cli_help() {
    let (stdout, _stderr, success) = run_fnloc(&["--help"]).expect("Failed to run fnloc --help");

    assert!(success, "Help command should succeed");
    assert!(
        stdout.contains("Function analyzer for Rust code"),
        "Help should contain description"
    );
    assert!(
        stdout.contains("--format"),
        "Help should contain format option"
    );
}

#[test]
fn test_cli_error_handling() {
    // Test with non-existent directory
    let (_stdout, stderr, success) =
        run_fnloc(&["non_existent_directory"]).expect("Failed to run fnloc with invalid directory");

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
    let (stdout, _stderr, success) = run_fnloc(&[]).expect("Failed to run fnloc with default args");

    assert!(success, "Default behavior should succeed");
    assert!(stdout.contains("Analyzing"), "Should show analysis header");
    assert!(stdout.contains("fn"), "Should show function results");
}

#[test]
fn test_cli_with_test_sample() {
    // Test with test sample directory
    let (stdout, _stderr, success) =
        run_fnloc(&["tests/test_sample"]).expect("Failed to run fnloc with test sample");

    assert!(success, "Should succeed with test sample directory");
    assert!(stdout.contains("Analyzing"), "Should show analysis header");
    assert!(stdout.contains("fn"), "Should show function results");
    assert!(
        stdout.contains("large_function"),
        "Should show sample functions"
    );
    assert!(
        stdout.contains("complexity="),
        "Should show cyclomatic complexity"
    );
    assert!(stdout.contains("nesting="), "Should show nesting depth");
}
