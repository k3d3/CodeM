use proptest::prelude::*;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;
use tempfile::TempDir;

use crate::types::GrepOptions;
use crate::grep::{grep_file, grep_codebase};
use regex::Regex;

proptest! {
    #[test]
    fn test_grep_file_with_varying_contexts(
        lines in proptest::collection::vec(r"[a-zA-Z0-9 ]{1,50}", 1..20)
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut file = NamedTempFile::new().unwrap();
            write!(file, "{}", lines.join("\n")).unwrap();

            let pattern = Regex::new("test").unwrap();

            // Try with different context sizes
            for context_size in 0..5 {
                let options = GrepOptions {
                    context_before: context_size,
                    context_after: context_size,
                    ..Default::default()
                };

                let _result = grep_file(&file, &pattern, &options).await.unwrap();
            }
        });
    }
}

#[tokio::test]
async fn test_grep_file_no_crashes() {
    let mut file = NamedTempFile::new().unwrap();

    // Test empty file
    let pattern = Regex::new("test").unwrap();
    let options = GrepOptions::default();
    grep_file(&file, &pattern, &options).await.unwrap();

    // Test single character
    write!(file, "a").unwrap();
    grep_file(&file, &pattern, &options).await.unwrap();

    // Test single line
    write!(file, "test line").unwrap();
    if let Ok(Some(file_match)) = grep_file(&file, &pattern, &options).await {
        assert_eq!(file_match.matches.len(), 1);
        assert_eq!(file_match.matches[0].line_number, 1);
    }

    // Test multiple lines
    writeln!(file).unwrap();
    writeln!(file, "another test").unwrap();
    if let Ok(Some(file_match)) = grep_file(&file, &pattern, &options).await {
        assert_eq!(file_match.matches.len(), 2);
        assert_eq!(file_match.matches[1].line_number, 2);
    }
}

#[tokio::test]
async fn test_grep_file_pattern_variations() {
    let test_patterns = vec![
        r"\w+",      // Word characters
        r"\d+",      // Digits
        r"[\s\S]+",  // Any character including newlines
        r"^test",    // Line start
        r"test$",    // Line end
        r"\btest\b", // Word boundaries
    ];

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "test123\ntest line\n123test\ntest\n").unwrap();

    let options = GrepOptions::default();
    for pattern_str in test_patterns {
        let pattern = Regex::new(pattern_str).unwrap();
        grep_file(&file, &pattern, &options).await.unwrap();
    }
}

#[tokio::test]
async fn test_grep_codebase_with_varied_structure() {
    let temp = TempDir::new().unwrap();

    // Create a more complex directory structure
    let dirs = vec![
        "",
        "dir1",
        "dir1/subdir1",
        "dir2",
        "dir2/subdir1/subsubdir1",
    ];

    for dir in dirs {
        let dir_path = temp.path().join(dir);
        if !dir.is_empty() {
            fs::create_dir_all(dir_path.clone()).unwrap();
        }

        // Add some files to each directory
        for i in 1..3 {
            let file_path = dir_path.join(format!("file{}.txt", i));
            let mut file = fs::File::create(file_path).unwrap();
            writeln!(file, "test line {}", i).unwrap();
            writeln!(file, "other content {}", i).unwrap();
        }
    }

    let pattern = Regex::new("test").unwrap();
    let file_matches = grep_codebase(
        temp.path(),
        &pattern,
        GrepOptions {
            context_before: 1,
            context_after: 1,
            ..Default::default()
        },
    ).await.unwrap();

    // We should find "test" in every file
    assert!(file_matches.len() > 0);
    for file_match in file_matches {
        assert!(file_match.matches.len() > 0);
    }
}
