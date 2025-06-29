use std::fs;
use std::path::Path;

/// Recursively finds all Rust files in a directory
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
