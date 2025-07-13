use crate::analyzer::FunctionAnalysisResult;
use crate::client::OutputFormat;

/// Handles the formatting and display of analysis results
pub struct OutputFormatter {
    format: OutputFormat,
}

impl OutputFormatter {
    /// Creates a new OutputFormatter instance with default table format
    pub fn new() -> Self {
        Self {
            format: OutputFormat::Table,
        }
    }

    /// Creates a new OutputFormatter instance with specified format
    pub fn with_format(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Displays the header information for the analysis
    pub fn display_analysis_header(&self, file_count: usize) {
        match self.format {
            OutputFormat::Table => {
                println!("Analyzing {file_count} Rust files...\n");
            }
            OutputFormat::Json => {
                // JSON header will be handled in the results output
            }
            OutputFormat::Csv => {
                println!(
                    "Function,Total Lines,Code Lines,Comment Lines,Empty Lines,Cyclomatic Complexity,Nesting Depth"
                );
            }
        }
    }

    /// Displays all analysis results sorted by code lines in descending order
    pub fn display_results_sorted_by_code(&self, results: &[FunctionAnalysisResult]) {
        let mut sorted_results = results.to_vec();
        sorted_results.sort_by(|a, b| b.code.cmp(&a.code));

        match self.format {
            OutputFormat::Table => {
                for result in sorted_results {
                    self.display_function_result_table(&result);
                }
            }
            OutputFormat::Json => {
                self.display_results_json(&sorted_results);
            }
            OutputFormat::Csv => {
                for result in sorted_results {
                    self.display_function_result_csv(&result);
                }
            }
        }
    }

    /// Displays the analysis result for a single function in table format
    fn display_function_result_table(&self, result: &FunctionAnalysisResult) {
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

    /// Displays the analysis result for a single function in CSV format
    fn display_function_result_csv(&self, result: &FunctionAnalysisResult) {
        println!(
            "{},{},{},{},{},{},{}",
            result.name,
            result.total,
            result.code,
            result.comment,
            result.empty,
            result.cyclomatic_complexity,
            result.nesting_depth
        );
    }

    /// Displays all results in JSON format
    fn display_results_json(&self, results: &[FunctionAnalysisResult]) {
        // For now, we'll use a simple JSON output
        // In a production system, we might want to use serde_json
        println!("[");
        for (i, result) in results.iter().enumerate() {
            let comma = if i < results.len() - 1 { "," } else { "" };
            println!(
                "  {{\"name\": \"{}\", \"total\": {}, \"code\": {}, \"comment\": {}, \"empty\": {}, \"complexity\": {}, \"nesting\": {}}}{}",
                result.name,
                result.total,
                result.code,
                result.comment,
                result.empty,
                result.cyclomatic_complexity,
                result.nesting_depth,
                comma
            );
        }
        println!("]");
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}
