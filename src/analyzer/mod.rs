//! Code analysis modules for function metrics
//!
//! This module contains all code analysis functionality including:
//! - Cyclomatic complexity calculation
//! - Nesting depth measurement
//! - Function line analysis and aggregation

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
pub use function_analyzer::{
    analyze_all_files,
    analyze_function_complete,
    analyze_function_lines, // Backward compatibility
    calculate_cyclomatic_complexity_from_source,
    calculate_nesting_depth_from_source,
    count_function_lines,
};
pub use function_extractor::{FunctionSpan, extract_function_spans, read_rust_file};
pub use nesting_depth::calculate_nesting_depth;
