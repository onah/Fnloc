//! Code analysis modules for function metrics
//!
//! This module contains all code analysis functionality including:
//! - Cyclomatic complexity calculation
//! - Nesting depth measurement
//! - Function line analysis and aggregation
//! - Function extraction from source code

use syn::{Item, parse_file};

/// Result of analyzing a function's line composition, complexity, and nesting
#[derive(Debug, Clone)]
pub struct FunctionAnalysisResult {
    pub name: String,
    pub total: usize,
    pub code: usize,
    pub comment: usize,
    pub empty: usize,
    pub cyclomatic_complexity: usize,
    pub nesting_depth: usize,
}

pub mod cyclomatic_complexity;
pub mod function_analyzer;
pub mod function_extractor;
pub mod nesting_depth;

// Re-export commonly used functions for convenience
pub use cyclomatic_complexity::calculate_cyclomatic_complexity;
pub use function_extractor::{FunctionSpan, extract_function_spans};
pub use nesting_depth::calculate_nesting_depth;

// Main interface functions are now defined directly in this module

// ============================================================================
// MAIN ANALYSIS INTERFACE FUNCTIONS
// ============================================================================

/// Counts lines in a function span (code, comment, empty lines)
/// Returns (total, code, comment, empty)
pub fn count_function_lines(func: &FunctionSpan) -> (usize, usize, usize, usize) {
    let mut code = 0;
    let mut comment = 0;
    let mut empty = 0;

    for line in &func.lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            empty += 1;
        } else if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            comment += 1;
        } else {
            code += 1;
        }
    }

    let total = func.lines.len();
    (total, code, comment, empty)
}

/// Calculates cyclomatic complexity for a specific function by name from source code
pub fn calculate_cyclomatic_complexity_from_source(source: &str, function_name: &str) -> usize {
    if let Ok(parsed) = parse_file(source) {
        for item in parsed.items {
            if let Item::Fn(func) = item {
                if func.sig.ident.to_string() == function_name {
                    return cyclomatic_complexity::calculate_cyclomatic_complexity(&func);
                }
            }
        }
    }
    1 // Default complexity for simple functions
}

/// Calculates nesting depth for a specific function by name from source code
pub fn calculate_nesting_depth_from_source(source: &str, function_name: &str) -> usize {
    if let Ok(parsed) = parse_file(source) {
        for item in parsed.items {
            if let Item::Fn(func) = item {
                if func.sig.ident.to_string() == function_name {
                    return nesting_depth::calculate_nesting_depth(&func);
                }
            }
        }
    }
    0 // Default nesting depth
}

/// Analyzes the line composition, cyclomatic complexity, and nesting depth of a function span
/// This is the main integration function that combines all metrics
pub fn analyze_function_complete(func: &FunctionSpan, source: &str) -> FunctionAnalysisResult {
    let (total, code, comment, empty) = count_function_lines(func);
    let cyclomatic_complexity = calculate_cyclomatic_complexity_from_source(source, &func.name);
    let nesting_depth = calculate_nesting_depth_from_source(source, &func.name);

    FunctionAnalysisResult {
        name: func.name.clone(),
        total,
        code,
        comment,
        empty,
        cyclomatic_complexity,
        nesting_depth,
    }
}

/// Backward compatibility alias for analyze_function_complete
/// @deprecated Use analyze_function_complete instead
pub fn analyze_function_lines(func: &FunctionSpan, source: &str) -> FunctionAnalysisResult {
    analyze_function_complete(func, source)
}
