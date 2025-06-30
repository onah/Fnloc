use walkdir::WalkDir;

/// Recursively finds all Rust files in a directory using walkdir
pub fn find_rust_files(dir: &str) -> Vec<String> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                None
            }
        })
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .collect()
}
