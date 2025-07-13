use crate::errors::{AnalysisError, AnalysisResult};
use syn::Item;

/// Represents a span of lines that contain a function
pub struct FunctionSpan {
    pub name: String,
    pub lines: Vec<String>,
}

/// Extracts function spans from source code using syn parser
pub fn extract_function_spans(source: &str) -> AnalysisResult<Vec<FunctionSpan>> {
    let lines: Vec<&str> = source.lines().collect();
    let parsed = syn::parse_file(source).map_err(|e| {
        AnalysisError::Io(std::io::Error::other(format!(
            "Failed to parse Rust source: {e}"
        )))
    })?;

    let mut spans = Vec::new();

    for item in parsed.items {
        if let Item::Fn(f) = item {
            let name = f.sig.ident.to_string();
            // syn doesn't provide line numbers directly, so we'll use a simpler approach
            // We'll find the function by name in the source lines
            if let Some((start, end)) = find_function_bounds(&lines, &name) {
                let slice: Vec<String> = lines[start..=end].iter().map(|s| s.to_string()).collect();

                spans.push(FunctionSpan { name, lines: slice });
            }
        }
    }

    Ok(spans)
}

/// Finds the start and end line indices of a function by name
fn find_function_bounds(lines: &[&str], fn_name: &str) -> Option<(usize, usize)> {
    let mut start = None;
    let mut brace_count = 0;
    let fn_pattern = format!("fn {fn_name}");

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
