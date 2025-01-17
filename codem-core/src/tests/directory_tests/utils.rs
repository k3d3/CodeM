use crate::types::TreeEntry;
use std::fs;
use std::path::{Path, PathBuf};

pub fn collect_paths(entry: &TreeEntry, paths: &mut Vec<PathBuf>) {
    paths.push(entry.path().clone());
    for child in &entry.children {
        collect_paths(child, paths);
    }
}

pub fn verify_sizes(entry: &TreeEntry, base_path: &Path) {
    if !entry.is_dir() {
        if let Some(entry_size) = entry.size() {
            let path = base_path.join(entry.path());
            let fs_size = fs::metadata(&path)
                .unwrap_or_else(|e| panic!("Failed to get metadata for {}: {}", path.display(), e))
                .len();
            println!("Verifying size for {}", entry.path().display());
            println!("  Entry size: {}", entry_size);
            println!("  FS size: {}", fs_size);
            assert_eq!(entry_size, fs_size, 
                "Size mismatch for {}: entry={}, fs={}",
                entry.path().display(), entry_size, fs_size);
        }
    }

    for child in &entry.children {
        verify_sizes(child, base_path);
    }
}

pub fn verify_entry_types(entry: &TreeEntry, base_path: &Path) {
    let path = base_path.join(entry.path());
    println!("Verifying type for {}", path.display());
    let metadata = fs::metadata(&path)
        .unwrap_or_else(|e| panic!("Failed to get metadata for {}: {}", path.display(), e));
    let is_dir = metadata.is_dir();
    assert_eq!(entry.is_dir(), is_dir, "Type mismatch for {}", path.display());
    
    assert_eq!(
        entry.entry_type(),
        if is_dir { "DIR" } else { "FILE" },
        "Entry type mismatch for {}", path.display()
    );

    for child in &entry.children {
        verify_entry_types(child, base_path);
    }
}

pub fn verify_pattern_matches(entry: &TreeEntry, pattern: &regex::Regex) {
    if !entry.is_dir() {
        let path_str = entry.path().to_string_lossy();
        assert!(pattern.is_match(&path_str), 
            "Path '{}' did not match pattern '{}'", path_str, pattern);
    }

    for child in &entry.children {
        verify_pattern_matches(child, pattern);
    }
}

pub fn verify_line_counts(entry: &TreeEntry, expected: &[(PathBuf, usize)]) {
    if let Some(stats) = entry.stats() {
        if let Some(actual_count) = stats.line_count {
            if let Some((path, expected_count)) = expected.iter()
                .find(|(p, _)| p == entry.path())
            {
                println!("Verifying line count for {}", path.display());
                println!("  Actual: {}", actual_count);
                println!("  Expected: {}", expected_count);
                assert_eq!(actual_count, *expected_count,
                    "Line count mismatch for {}: actual={}, expected={}",
                    path.display(), actual_count, expected_count);
            }
        }
    }

    for child in &entry.children {
        verify_line_counts(child, expected);
    }
}