use proptest::prelude::*;
use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::TestRunner;
use regex::Regex;
use tempfile::TempDir;
use std::fs;

use crate::grep::{grep_file, grep_codebase};
use crate::types::GrepOptions;

// Strategy to generate text with a mix of matching and non-matching lines
fn text_strategy(context_before: usize, context_after: usize) -> impl Strategy<Value = String> {
    let text_parts = "[a-zA-Z ]{0,20}";
    prop::collection::vec(text_parts, 1..10)
        .prop_map(move |parts| {
            // Create base lines
            let lines: Vec<String> = parts.into_iter().map(|part| {
                if Just(true).new_tree(&mut TestRunner::default())
                    .unwrap()
                    .current()
                {
                    format!("{}target{}", part, part)
                } else {
                    part
                }
            }).collect();
            
            // Add padding between lines to avoid context overlap
            let mut result = Vec::new();
            let min_padding = context_before.max(context_after);
            for (i, line) in lines.into_iter().enumerate() {
                if i > 0 && min_padding > 0 {
                    // Add padding before this line
                    for _ in 0..min_padding {
                        result.push(String::from("padding line"));
                    }
                }
                result.push(line);
            }
            result.join("\n")
        })
}

// Strategy to generate directory paths
fn directory_name_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9]{1,10}".prop_map(|s| format!("dir_{}", s))
}

// Strategy to generate file names with extensions
fn file_name_strategy() -> impl Strategy<Value = String> {
    ("[a-zA-Z0-9]{1,10}", prop::sample::select(vec![".txt", ".rs"]))
        .prop_map(|(name, ext)| format!("file_{}{}", name, ext))
}

proptest! {
    #[test]
    fn test_grep_match_positions(
        content in text_strategy(0, 0)
    ) {
        // Create temp file with content
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");
        fs::write(&file, &content).unwrap();

        let pattern = Regex::new("target").unwrap();
        if let Ok(matches) = grep_file(&file, &pattern, &GrepOptions::default()) {
            for grep_match in matches {
                // Match positions should be within line bounds
                prop_assert!(grep_match.match_start < grep_match.line.len());
                prop_assert!(grep_match.match_end <= grep_match.line.len());
                
                // Extract all values before the assertions to avoid moves
                let line = grep_match.line.clone();
                let line_content = grep_match.line_content.clone();
                let matched_text = line[grep_match.match_start..grep_match.match_end].to_string();
                
                // Now do the assertions
                prop_assert_eq!(&line, &line_content);
                prop_assert_eq!(&matched_text, "target");

                // Context lines shouldn't contain matches
                for line in &grep_match.context_before {
                    prop_assert!(!line.contains("target"));
                }
                for line in &grep_match.context_after {
                    prop_assert!(!line.contains("target"));
                }
            }
        }
    }

    #[test]
    fn test_grep_context_lines(
        content_before in 0..5usize,
        content_after in 0..5usize
    ) {
        let content = text_strategy(content_before, content_after)
            .new_tree(&mut TestRunner::default())
            .unwrap()
            .current();

        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");
        fs::write(&file, &content).unwrap();

        let pattern = Regex::new("target").unwrap();
        let options = GrepOptions {
            context_before: content_before,
            context_after: content_after,
            ..Default::default()
        };
        if let Ok(matches) = grep_file(&file, &pattern, &options) {
            let content_lines: Vec<&str> = content.lines().collect();
            
            for grep_match in matches {
                // Check line index and context bounds
                let line_idx = grep_match.line_number - 1;
                let remaining_lines = content_lines.len().saturating_sub(line_idx + 1);
                let max_context = remaining_lines.min(content_after);

                // When no context is requested, there should be no context lines
                if content_before == 0 {
                    prop_assert_eq!(grep_match.context_before.len(), 0);
                }
                if content_after == 0 {
                    prop_assert_eq!(grep_match.context_after.len(), 0);
                    continue;
                }

                // Otherwise, check that context is within bounds
                if content_before > 0 {
                    prop_assert!(grep_match.context_before.len() <= content_before);
                    let context_start = line_idx.saturating_sub(content_before);
                    for (i, context_line) in grep_match.context_before.iter().enumerate() {
                        prop_assert_eq!(context_line, content_lines[context_start + i]);
                    }
                }

                if content_after > 0 {
                    prop_assert!(grep_match.context_after.len() <= max_context);
                    let after_start = line_idx + 1;
                    if after_start < content_lines.len() {
                        let after_end = content_lines.len().min(after_start + content_after);
                        for (i, context_line) in grep_match.context_after.iter().enumerate() {
                            if after_start + i < after_end {
                                prop_assert_eq!(context_line, content_lines[after_start + i]);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_grep_codebase_structure(
        file_count in 1..5usize,
        dir_count in 1..3usize,
        pattern in "[a-zA-Z]{1,10}"
    ) {
        let temp = TempDir::new().unwrap();
        
        // Create directory structure
        let mut all_paths = Vec::new();
        all_paths.push(temp.path().to_path_buf());
        
        // Create directories
        for _ in 0..dir_count {
            let dir_name = directory_name_strategy()
                .new_tree(&mut TestRunner::default())
                .unwrap()
                .current();
            let idx_tree = (0..all_paths.len())
                .prop_map(|i| i)
                .new_tree(&mut TestRunner::default())
                .unwrap();
            let parent_idx = idx_tree.current();
            let parent = &all_paths[parent_idx];
            let dir_path = parent.join(dir_name);
            fs::create_dir_all(&dir_path).unwrap();
            all_paths.push(dir_path);
        }

        // Create files
        for _ in 0..file_count {
            let idx_tree = (0..all_paths.len())
                .prop_map(|i| i)
                .new_tree(&mut TestRunner::default())
                .unwrap();
            let parent_idx = idx_tree.current();
            let parent = &all_paths[parent_idx];
            let file_name = file_name_strategy()
                .new_tree(&mut TestRunner::default())
                .unwrap()
                .current();
            let file_path = parent.join(&file_name);
            
            let content = text_strategy(0, 0)
                .new_tree(&mut TestRunner::default())
                .unwrap()
                .current();
                
            fs::write(&file_path, content).unwrap();
        }

        let grep_pattern = Regex::new(&pattern).unwrap();
        if let Ok(matches) = grep_codebase(
            temp.path(),
            &grep_pattern,
            GrepOptions {
                file_pattern: Some("*.txt".into()),
                ..Default::default()
            },
        ) {
            // All matches should be from .txt files
            for grep_match in matches {
                prop_assert_eq!(grep_match.path.extension().unwrap(), "txt");
                // Path should be under temp directory
                prop_assert!(grep_match.path.starts_with(temp.path()));
            }
        }
    }
}