use std::fs;
use std::path::Path;
use syn::Item;

pub struct FunctionAnalysisResult {
    pub name: String,
    pub total: usize,
    pub code: usize,
    pub comment: usize,
    pub empty: usize,
    pub start_line: usize,
    pub end_line: usize,
}

pub struct FunctionSpan {
    pub name: String,
    pub start_line: usize, // 1-based
    pub end_line: usize,   // inclusive, 1-based
    pub lines: Vec<String>,
}

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
        start_line: func.start_line,
        end_line: func.end_line,
    }
}

pub fn find_rust_files(dir: &str) -> Vec<String> {
    let mut rust_files = Vec::new();

    fn visit_dir(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dir(&path, files)?;
                } else if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        if let Some(path_str) = path.to_str() {
                            files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    if let Err(e) = visit_dir(Path::new(dir), &mut rust_files) {
        eprintln!("Error reading directory {}: {}", dir, e);
    }

    rust_files
}

pub fn extract_function_spans(source: &str) -> Vec<FunctionSpan> {
    let lines: Vec<&str> = source.lines().collect();
    let parsed = syn::parse_file(source).expect("parse failed");

    let mut spans = Vec::new();

    for item in parsed.items {
        if let Item::Fn(f) = item {
            let name = f.sig.ident.to_string();
            // syn doesn't provide line numbers directly, so we'll use a simpler approach
            // We'll find the function by name in the source lines
            if let Some((start, end)) = find_function_bounds(&lines, &name) {
                let slice: Vec<String> = lines[start..=end].iter().map(|s| s.to_string()).collect();

                spans.push(FunctionSpan {
                    name,
                    start_line: start + 1,
                    end_line: end + 1,
                    lines: slice,
                });
            }
        }
    }

    spans
}

fn find_function_bounds(lines: &[&str], fn_name: &str) -> Option<(usize, usize)> {
    let mut start = None;
    let mut brace_count = 0;
    let fn_pattern = format!("fn {}", fn_name);

    for (i, line) in lines.iter().enumerate() {
        if start.is_none() && line.contains(&fn_pattern) {
            start = Some(i);
        }

        if start.is_some() {
            // Count braces to find the end of the function
            for ch in line.chars() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return Some((start?, i));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    None
}

pub fn read_rust_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

pub fn analyze_file_functions(path: &str) -> Vec<FunctionAnalysisResult> {
    let source = read_rust_file(path);
    let function_spans = extract_function_spans(&source);

    function_spans
        .iter()
        .map(|f| analyze_function_lines(f))
        .collect()
}

fn main() {
    let files = find_rust_files("./src");

    for file in files {
        println!("{}", file);
        let results = analyze_file_functions(&file);
        for r in results {
            println!(
                "  - fn {}: total={} lines, code={}, comment={}, empty={}",
                r.name, r.total, r.code, r.comment, r.empty
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_line_counting() {
        let source = vec![
            "// comment".to_string(),
            "fn hello() {".to_string(),
            "  println!(\"Hello\");".to_string(),
            "".to_string(),
            "// another".to_string(),
            "}".to_string(),
        ];

        let span = FunctionSpan {
            name: "hello".to_string(),
            start_line: 2,
            end_line: 6,
            lines: source[0..6].to_vec(),
        };
        let result = analyze_function_lines(&span);

        // デバッグ出力
        println!("Lines:");
        for (i, line) in span.lines.iter().enumerate() {
            let trimmed = line.trim();
            let line_type = if trimmed.is_empty() {
                "empty"
            } else if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                "comment"
            } else {
                "code"
            };
            println!("  {}: '{}' -> {}", i, line, line_type);
        }
        println!(
            "Result: code={}, comment={}, empty={}",
            result.code, result.comment, result.empty
        );

        assert_eq!(result.code, 2);
        assert_eq!(result.comment, 2);
        assert_eq!(result.empty, 1);
    }
}
