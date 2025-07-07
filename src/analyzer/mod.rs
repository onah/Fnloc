//! Code analysis modules for function metrics
//!
//! This module contains all code analysis functionality including:
//! - Cyclomatic complexity calculation
//! - Nesting depth measurement
//! - Function line analysis and aggregation

pub mod cyclomatic_complexity;
pub mod nesting_depth;
pub mod function_analyzer;

// Re-export commonly used functions for convenience
pub use cyclomatic_complexity::calculate_cyclomatic_complexity;
pub use nesting_depth::calculate_nesting_depth;
pub use function_analyzer::{
    analyze_function_lines, 
    analyze_all_files, 
    FunctionAnalysisResult
};
