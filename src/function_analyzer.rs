use crate::function_extractor::{FunctionSpan, extract_function_spans, read_rust_file};

/// Result of analyzing a function's line composition
#[derive(Debug, Clone)]
pub struct FunctionAnalysisResult {
    pub name: String,
    pub total: usize,
    pub code: usize,
    pub comment: usize,
    pub empty: usize,
}

/// Analyzes the line composition of a function span
pub fn analyze_function_lines(func: &FunctionSpan) -> FunctionAnalysisResult {
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

    FunctionAnalysisResult {
        name: func.name.clone(),
        total: func.lines.len(),
        code,
        comment,
        empty,
    }
}

/// Analyzes all functions in a Rust file and returns analysis results
pub fn analyze_file_functions(path: &str) -> Vec<FunctionAnalysisResult> {
    let source = read_rust_file(path);
    let function_spans = extract_function_spans(&source);

    function_spans.iter().map(analyze_function_lines).collect()
}

/// Analyzes all functions across multiple files and returns results sorted by code lines
pub fn analyze_all_files_sorted_by_code(file_paths: &[String]) -> Vec<FunctionAnalysisResult> {
    let mut all_results = Vec::new();

    for path in file_paths {
        let mut file_results = analyze_file_functions(path);
        // Add file path information to each result for context
        for result in &mut file_results {
            // We'll modify the name to include the file path
            result.name = format!("{}::{}", path, result.name);
        }
        all_results.extend(file_results);
    }

    // Sort all functions by code lines in descending order
    all_results.sort_by(|a, b| b.code.cmp(&a.code));
    all_results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function_extractor::FunctionSpan;

    /// Helper function to create a FunctionSpan for testing
    fn create_test_function_span(name: &str, lines: Vec<&str>) -> FunctionSpan {
        FunctionSpan {
            name: name.to_string(),
            lines: lines.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_analyze_function_lines_code_only() {
        let func_span = create_test_function_span(
            "test_function",
            vec![
                "fn test_function() {",
                "    let x = 5;",
                "    let y = 10;",
                "    x + y",
                "}",
            ],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "test_function");
        assert_eq!(result.total, 5);
        assert_eq!(result.code, 5);
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 0);
    }

    #[test]
    fn test_analyze_function_lines_mixed_content() {
        let func_span = create_test_function_span(
            "mixed_function",
            vec![
                "fn mixed_function() {",
                "    // This is a comment",
                "    let x = 5;",
                "",
                "    /* Block comment */",
                "    let y = 10;",
                "",
                "    x + y",
                "}",
            ],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "mixed_function");
        assert_eq!(result.total, 9);
        assert_eq!(result.code, 5); // function signature, 2 let statements, expression, closing brace
        assert_eq!(result.comment, 2); // single line and block comment
        assert_eq!(result.empty, 2); // two empty lines
    }

    #[test]
    fn test_analyze_function_lines_whitespace_only_lines() {
        let func_span = create_test_function_span(
            "whitespace_function",
            vec![
                "fn whitespace_function() {",
                "    let x = 5;",
                "   ", // whitespace only - should be counted as empty
                "\t\t", // tabs only - should be counted as empty
                "    return x;",
                "}",
            ],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "whitespace_function");
        assert_eq!(result.total, 6);
        assert_eq!(result.code, 4); // function signature, let statement, return, closing brace
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 2); // two lines with only whitespace
    }

    #[test]
    fn test_analyze_function_lines_different_comment_styles() {
        let func_span = create_test_function_span(
            "comment_styles_function",
            vec![
                "fn comment_styles_function() {",
                "    // Single line comment",
                "    /* Block comment on single line */",
                "    let x = 5; // Inline comment is treated as code",
                "    /*",
                "     * Multi-line comment start",
                "     */",
                "}",
            ],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "comment_styles_function");
        assert_eq!(result.total, 8);
        assert_eq!(result.code, 5); // function signature, let statement, multi-line comment middle/end, closing brace
        assert_eq!(result.comment, 3); // lines starting with "//" or "/*"
        assert_eq!(result.empty, 0);
    }

    #[test]
    fn test_analyze_function_lines_comments_and_empty_only() {
        let func_span = create_test_function_span(
            "comments_only_function",
            vec![
                "// This function has no actual code",
                "/* Just comments and empty lines */",
                "",
                "// Another comment",
                "",
                "/* Final comment */",
            ],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "comments_only_function");
        assert_eq!(result.total, 6);
        assert_eq!(result.code, 0);
        assert_eq!(result.comment, 4);
        assert_eq!(result.empty, 2);
    }

    #[test]
    fn test_analyze_function_lines_single_line_function() {
        let func_span = create_test_function_span(
            "single_line",
            vec!["fn single_line() { 42 }"],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "single_line");
        assert_eq!(result.total, 1);
        assert_eq!(result.code, 1);
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 0);
    }

    #[test]
    fn test_analyze_function_lines_empty_function() {
        let func_span = create_test_function_span(
            "empty_function",
            vec![
                "fn empty_function() {",
                "",
                "",
                "}",
            ],
        );

        let result = analyze_function_lines(&func_span);

        assert_eq!(result.name, "empty_function");
        assert_eq!(result.total, 4);
        assert_eq!(result.code, 2); // function signature and closing brace
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 2);
    }
}
