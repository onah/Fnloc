use std::io;
use walkdir::WalkDir;

/// Recursively finds all Rust files in a directory using walkdir
/// Returns a Result containing the list of Rust file paths or an error
pub fn find_rust_files(dir: &str) -> Result<Vec<String>, io::Error> {
    let mut rust_files = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry =
            entry.map_err(|e| io::Error::other(format!("Error reading directory entry: {}", e)))?;

        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            if let Some(path_str) = entry.path().to_str() {
                rust_files.push(path_str.to_string());
            }
        }
    }

    Ok(rust_files)
}
