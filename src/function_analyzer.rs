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
