use thiserror::Error;

/// Errors that can occur during function analysis
#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Directory not accessible: {directory}")]
    DirectoryNotAccessible { directory: String },

    #[error("No Rust files found in directory: {directory}")]
    NoRustFiles { directory: String },
}

/// Result type alias for convenience
pub type AnalysisResult<T> = Result<T, AnalysisError>;
