//! Fnloc - Function Line Counter for Rust
//!
//! A tool for analyzing Rust source code and counting lines of code,
//! comments, and empty lines per function.

pub mod errors;
pub mod file_scanner;
pub mod function_analyzer;
pub mod function_extractor;
pub mod output_formatter;

// Re-export commonly used types for convenience
pub use errors::{AnalysisError, AnalysisResult};
pub use function_analyzer::FunctionAnalysisResult;
