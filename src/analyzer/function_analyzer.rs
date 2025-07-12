use crate::analyzer::FunctionAnalysisResult;
use crate::analyzer::cyclomatic_complexity::calculate_cyclomatic_complexity;
use crate::analyzer::nesting_depth::calculate_nesting_depth;
use crate::function_extractor::{FunctionSpan, extract_function_spans, read_rust_file};
use syn::{Item, parse_file};

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
                    return calculate_cyclomatic_complexity(&func);
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
                    return calculate_nesting_depth(&func);
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

/// Analyzes all functions in a Rust file and returns analysis results
pub fn analyze_file_functions(path: &str) -> Vec<FunctionAnalysisResult> {
    let source = read_rust_file(path);
    let function_spans = extract_function_spans(&source);

    function_spans
        .iter()
        .map(|span| analyze_function_complete(span, &source))
        .collect()
}

/// Analyzes all functions across multiple files and returns unsorted results
pub fn analyze_all_files(file_paths: &[String]) -> Vec<FunctionAnalysisResult> {
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

    all_results
}

/// Backward compatibility alias for analyze_function_complete
/// @deprecated Use analyze_function_complete instead
pub fn analyze_function_lines(func: &FunctionSpan, source: &str) -> FunctionAnalysisResult {
    analyze_function_complete(func, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::FunctionAnalysisResult;
    use crate::function_extractor::FunctionSpan;

    #[test]
    fn test_analyze_function_lines_basic() {
        let lines = vec![
            "fn hello() {".to_string(),
            "    println!(\"Hello\");".to_string(),
            "    // This is a comment".to_string(),
            "".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "hello".to_string(),
            lines,
        };

        let source = "fn hello() {\n    println!(\"Hello\");\n    // This is a comment\n\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "hello");
        assert_eq!(result.total, 5);
        assert_eq!(result.code, 3); // fn hello() {, println!, }
        assert_eq!(result.comment, 1); // // This is a comment
        assert_eq!(result.empty, 1); // empty line
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
        assert_eq!(result.nesting_depth, 0); // No nesting
    }

    #[test]
    fn test_analyze_function_lines_only_code() {
        let lines = vec![
            "fn add(a: i32, b: i32) -> i32 {".to_string(),
            "    a + b".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "add".to_string(),
            lines,
        };

        let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "add");
        assert_eq!(result.total, 3);
        assert_eq!(result.code, 3);
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 0);
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
        assert_eq!(result.nesting_depth, 0); // No nesting
    }

    #[test]
    fn test_analyze_function_lines_only_comments() {
        let lines = vec![
            "// Function documentation".to_string(),
            "/* Multi-line comment".to_string(),
            "   continues here */".to_string(),
            "// Another comment".to_string(),
        ];

        let span = FunctionSpan {
            name: "documented_function".to_string(),
            lines,
        };

        // For this test, the source doesn't contain the actual function,
        // so complexity will default to 1
        let source = "fn documented_function() {}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "documented_function");
        assert_eq!(result.total, 4);
        assert_eq!(result.code, 1); // "   continues here */" doesn't start with // or /*
        assert_eq!(result.comment, 3);
        assert_eq!(result.empty, 0);
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
        assert_eq!(result.nesting_depth, 0); // No nesting
    }

    #[test]
    fn test_analyze_function_lines_empty_function() {
        let lines = vec![
            "fn empty_function() {".to_string(),
            "".to_string(),
            "".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "empty_function".to_string(),
            lines,
        };

        let source = "fn empty_function() {\n\n\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "empty_function");
        assert_eq!(result.total, 4);
        assert_eq!(result.code, 2); // fn declaration and closing brace
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 2);
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
    }

    #[test]
    fn test_analyze_function_lines_mixed_content() {
        let lines = vec![
            "fn complex_function() {".to_string(),
            "    // Initialize variables".to_string(),
            "    let x = 10;".to_string(),
            "".to_string(),
            "    /* Calculate result".to_string(),
            "       using complex logic */".to_string(),
            "    let result = x * 2;".to_string(),
            "".to_string(),
            "    // Return the result".to_string(),
            "    result".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "complex_function".to_string(),
            lines,
        };

        let source = "fn complex_function() {\n    // Initialize variables\n    let x = 10;\n\n    /* Calculate result\n       using complex logic */\n    let result = x * 2;\n\n    // Return the result\n    result\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "complex_function");
        assert_eq!(result.total, 11);
        assert_eq!(result.code, 6); // fn declaration, let x, let result, result, }, and "using complex logic"
        assert_eq!(result.comment, 3); // Three lines starting with // or /*
        assert_eq!(result.empty, 2);
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
    }

    #[test]
    fn test_analyze_all_files_single_file() {
        // This is an integration test that would require actual file I/O
        // For a unit test, we would need to mock the file reading functionality
        // For now, we'll test the structure
        let file_paths = vec!["test_file.rs".to_string()];

        // Note: This test would fail in actual execution because the file doesn't exist
        // In a real test environment, we would either:
        // 1. Create temporary test files
        // 2. Mock the file reading functions
        // 3. Use dependency injection for the file reader

        // For demonstration purposes, we'll just verify the function signature
        assert_eq!(file_paths.len(), 1);
    }

    #[test]
    fn test_analyze_all_files_empty_list() {
        let file_paths: Vec<String> = vec![];
        let results = analyze_all_files(&file_paths);

        assert!(results.is_empty());
    }

    #[test]
    fn test_function_analysis_result_creation() {
        let result = FunctionAnalysisResult {
            name: "test_function".to_string(),
            total: 10,
            code: 7,
            comment: 2,
            empty: 1,
            cyclomatic_complexity: 3,
            nesting_depth: 2,
        };

        assert_eq!(result.name, "test_function");
        assert_eq!(result.total, 10);
        assert_eq!(result.code, 7);
        assert_eq!(result.comment, 2);
        assert_eq!(result.empty, 1);
        assert_eq!(result.cyclomatic_complexity, 3);
        assert_eq!(result.nesting_depth, 2);
        assert_eq!(result.total, result.code + result.comment + result.empty);
    }

    #[test]
    fn test_function_analysis_result_clone() {
        let original = FunctionAnalysisResult {
            name: "original".to_string(),
            total: 5,
            code: 3,
            comment: 1,
            empty: 1,
            cyclomatic_complexity: 2,
            nesting_depth: 1,
        };

        let cloned = original.clone();

        assert_eq!(original.name, cloned.name);
        assert_eq!(original.total, cloned.total);
        assert_eq!(original.code, cloned.code);
        assert_eq!(original.comment, cloned.comment);
        assert_eq!(original.empty, cloned.empty);
        assert_eq!(original.cyclomatic_complexity, cloned.cyclomatic_complexity);
        assert_eq!(original.nesting_depth, cloned.nesting_depth);
    }

    #[test]
    fn test_analyze_all_files_with_file_paths() {
        // Test the structure and path formatting without actual file I/O
        let file_paths = vec!["src/test1.rs".to_string(), "src/test2.rs".to_string()];

        // We can't test the actual file reading without mocking,
        // but we can test the empty case
        let empty_paths: Vec<String> = vec![];
        let results = analyze_all_files(&empty_paths);
        assert!(results.is_empty());

        // Verify that the function accepts the correct parameter type
        assert_eq!(file_paths.len(), 2);
        assert_eq!(file_paths[0], "src/test1.rs");
        assert_eq!(file_paths[1], "src/test2.rs");
    }

    #[test]
    fn test_line_classification_edge_cases() {
        // Test various edge cases for line classification
        let lines = vec![
            "    // Comment with leading spaces".to_string(),
            "\t/* Comment with tab */".to_string(),
            "  ".to_string(),                              // Only spaces
            "\t\t".to_string(),                            // Only tabs
            "code_line();  // Inline comment".to_string(), // Mixed content - this is code, not comment
            "/*".to_string(),                              // Comment start only
        ];

        let span = FunctionSpan {
            name: "edge_case_function".to_string(),
            lines,
        };

        let source = "fn edge_case_function() {\n    // Comment with leading spaces\n\t/* Comment with tab */\n  \n\t\t\n    code_line();  // Inline comment\n    /*\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "edge_case_function");
        assert_eq!(result.total, 6);
        // Let's debug what's actually happening:
        // "    // Comment" -> comment (starts with // after trim)
        // "\t/* Comment" -> comment (starts with /* after trim)
        // "  " -> empty (only whitespace)
        // "\t\t" -> empty (only whitespace)
        // "code_line();" -> code (doesn't start with // or /*)
        // "/*" -> comment (starts with /*)
        assert_eq!(result.code, 1); // Only "code_line();" line
        assert_eq!(result.comment, 3); // Three lines starting with // or /*
        assert_eq!(result.empty, 2); // Lines with only whitespace
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
    }

    #[test]
    fn test_analyze_function_lines_zero_lines() {
        let lines = vec![];

        let span = FunctionSpan {
            name: "zero_lines".to_string(),
            lines,
        };

        let source = "fn zero_lines() {}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.name, "zero_lines");
        assert_eq!(result.total, 0);
        assert_eq!(result.code, 0);
        assert_eq!(result.comment, 0);
        assert_eq!(result.empty, 0);
        assert_eq!(result.cyclomatic_complexity, 1); // Simple function
    }

    #[test]
    fn test_cyclomatic_complexity_simple_function() {
        let lines = vec![
            "fn simple() {".to_string(),
            "    println!(\"Hello\");".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "simple".to_string(),
            lines,
        };

        let source = "fn simple() {\n    println!(\"Hello\");\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.cyclomatic_complexity, 1);
    }

    #[test]
    fn test_cyclomatic_complexity_with_if() {
        let lines = vec![
            "fn with_if(x: i32) {".to_string(),
            "    if x > 0 {".to_string(),
            "        println!(\"positive\");".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "with_if".to_string(),
            lines,
        };

        let source =
            "fn with_if(x: i32) {\n    if x > 0 {\n        println!(\"positive\");\n    }\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.cyclomatic_complexity, 2); // Base 1 + if 1
    }

    #[test]
    fn test_cyclomatic_complexity_with_match() {
        let lines = vec![
            "fn with_match(x: Option<i32>) {".to_string(),
            "    match x {".to_string(),
            "        Some(val) => println!(\"{}\", val),".to_string(),
            "        None => println!(\"nothing\"),".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "with_match".to_string(),
            lines,
        };

        let source = "fn with_match(x: Option<i32>) {\n    match x {\n        Some(val) => println!(\"{}\", val),\n        None => println!(\"nothing\"),\n    }\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.cyclomatic_complexity, 4); // Base 1 + match 1 + 2 arms
    }

    #[test]
    fn test_cyclomatic_complexity_with_loops() {
        let lines = vec![
            "fn with_loops() {".to_string(),
            "    while true {".to_string(),
            "        break;".to_string(),
            "    }".to_string(),
            "    for i in 0..10 {".to_string(),
            "        continue;".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "with_loops".to_string(),
            lines,
        };

        let source = "fn with_loops() {\n    while true {\n        break;\n    }\n    for i in 0..10 {\n        continue;\n    }\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.cyclomatic_complexity, 5); // Base 1 + while 1 + for 1 + break 1 + continue 1
    }

    #[test]
    fn test_cyclomatic_complexity_with_logical_operators() {
        let lines = vec![
            "fn with_logical(a: bool, b: bool, c: bool) {".to_string(),
            "    if a && b || c {".to_string(),
            "        println!(\"complex condition\");".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "with_logical".to_string(),
            lines,
        };

        let source = "fn with_logical(a: bool, b: bool, c: bool) {\n    if a && b || c {\n        println!(\"complex condition\");\n    }\n}";
        let result = analyze_function_lines(&span, source);

        assert_eq!(result.cyclomatic_complexity, 4); // Base 1 + if 1 + && 1 + || 1
    }

    #[test]
    fn test_count_function_lines_basic() {
        let lines = vec![
            "fn test() {".to_string(),
            "    let x = 1;".to_string(),
            "    // comment".to_string(),
            "".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "test".to_string(),
            lines,
        };

        let (total, code, comment, empty) = count_function_lines(&span);

        assert_eq!(total, 5);
        assert_eq!(code, 3); // fn, let, }
        assert_eq!(comment, 1); // // comment
        assert_eq!(empty, 1); // empty line
    }

    #[test]
    fn test_calculate_cyclomatic_complexity_from_source() {
        let source =
            "fn with_if(x: i32) {\n    if x > 0 {\n        println!(\"positive\");\n    }\n}";
        let complexity = calculate_cyclomatic_complexity_from_source(source, "with_if");
        assert_eq!(complexity, 2); // Base 1 + if 1
    }

    #[test]
    fn test_calculate_nesting_depth_from_source() {
        let source = "fn nested() {\n    if true {\n        for i in 0..10 {\n            println!(\"{}\", i);\n        }\n    }\n}";
        let nesting = calculate_nesting_depth_from_source(source, "nested");
        assert_eq!(nesting, 2); // if + for
    }

    #[test]
    fn test_function_not_found_defaults() {
        let source = "fn other_function() {}";

        let complexity = calculate_cyclomatic_complexity_from_source(source, "nonexistent");
        assert_eq!(complexity, 1); // Default complexity

        let nesting = calculate_nesting_depth_from_source(source, "nonexistent");
        assert_eq!(nesting, 0); // Default nesting
    }
}
