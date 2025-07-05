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

    /// Displays all analysis results sorted by code lines in descending order
    pub fn display_results_sorted_by_code(&self, results: &[FunctionAnalysisResult]) {
        let mut sorted_results = results.to_vec();
        sorted_results.sort_by(|a, b| b.code.cmp(&a.code));

        for result in sorted_results {
            self.display_function_result(&result);
        }
    }

    /// Displays the analysis result for a single function
    fn display_function_result(&self, result: &FunctionAnalysisResult) {
        println!(
            "  - fn {}: total={} lines, code={}, comment={}, empty={}, complexity={}, nesting={}",
            result.name,
            result.total,
            result.code,
            result.comment,
            result.empty,
            result.cyclomatic_complexity,
            result.nesting_depth
        );
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}
