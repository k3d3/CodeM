use crate::types::TreeEntry;
use std::path::PathBuf;

pub fn verify_pattern_matches(entry: &TreeEntry, pattern: &regex::Regex) {
    if !entry.is_dir() {
        let path_str = entry.path().to_string_lossy();
        // Changed comment style from /* */ to //
        // Matching pattern check
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