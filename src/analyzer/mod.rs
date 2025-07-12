//! Code analysis modules for function metrics
//!
//! This module contains all code analysis functionality including:
//! - Cyclomatic complexity calculation
//! - Nesting depth measurement
//! - Function line analysis and aggregation

pub mod cyclomatic_complexity;
pub mod function_analyzer;
pub mod nesting_depth;

// Re-export commonly used functions for convenience
pub use cyclomatic_complexity::calculate_cyclomatic_complexity;
pub use function_analyzer::{
    FunctionAnalysisResult,
    analyze_all_files,
    analyze_function_complete,
    analyze_function_lines, // Backward compatibility
    calculate_cyclomatic_complexity_from_source,
    calculate_nesting_depth_from_source,
    count_function_lines,
};
pub use nesting_depth::calculate_nesting_depth;
