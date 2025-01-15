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
            let fs_size = fs::metadata(base_path.join(entry.path()))
                .unwrap()
                .len();
            assert_eq!(entry_size, fs_size);
        }
    }

    for child in &entry.children {
        verify_sizes(child, base_path);
    }
}

pub fn verify_entry_types(entry: &TreeEntry, base_path: &Path) {
    let path = base_path.join(entry.path());
    let is_dir = fs::metadata(&path).unwrap().is_dir();
    assert_eq!(entry.is_dir(), is_dir);
    
    if let Some(entry_type) = entry.entry_type() {
        assert_eq!(
            entry_type,
            if is_dir { "directory" } else { "file" }
        );
    }

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
            if let Some((_, expected_count)) = expected.iter()
                .find(|(path, _)| path == entry.path())
            {
                assert_eq!(actual_count, *expected_count);
            }
        }
    }

    for child in &entry.children {
        verify_line_counts(child, expected);
    }
}