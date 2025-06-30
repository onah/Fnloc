use crate::function_analyzer::FunctionAnalysisResult;

/// Handles the formatting and display of analysis results
pub struct OutputFormatter;

impl OutputFormatter {
    /// Creates a new OutputFormatter instance
    pub fn new() -> Self {
        Self
    }

    /// Displays the header information for the analysis
    pub fn display_analysis_header(&self, file_count: usize) {
        println!("Analyzing {} Rust files...\n", file_count);
    }

    /// Displays all analysis results in a formatted manner
    pub fn display_results(&self, results: &[FunctionAnalysisResult]) {
        for result in results {
            self.display_function_result(result);
        }
    }

    /// Displays the analysis result for a single function
    fn display_function_result(&self, result: &FunctionAnalysisResult) {
        println!(
            "  - fn {}: total={} lines, code={}, comment={}, empty={}",
            result.name, result.total, result.code, result.comment, result.empty
        );
    }

    /// Displays a message when no files are found
    pub fn display_no_files_message(&self, directory: &str) {
        println!("No Rust files found in directory: {}", directory);
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}
