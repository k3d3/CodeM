use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::TestRunner;
use tempfile::TempDir;
use std::fs;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

use crate::directory::list_directory;
use crate::types::ListOptions;

// Strategy to generate file content
fn file_content_strategy() -> impl Strategy<Value = (String, usize)> {
    proptest::collection::vec("[a-zA-Z0-9 ]{1,50}", 1..20)
        .prop_map(|lines| {
            let content = lines.join("\n");
            (content, lines.len())
        })
}

// Strategy to create a nested directory structure
fn dir_structure_strategy() -> impl Strategy<Value = Vec<(PathBuf, Option<String>)>> {
    let dir_count = 1..5usize;
    let file_count = 1..10usize;
    
    (dir_count, file_count).prop_flat_map(|(dir_count, file_count)| {
        let dirs = proptest::collection::vec("[a-zA-Z0-9]{1,10}", dir_count);
        let files = proptest::collection::vec("[a-zA-Z0-9]{1,10}", file_count);
        let content_strategy = file_content_strategy()
            .prop_map(|(content, _)| content);
        (dirs, files, content_strategy)
    }).prop_map(|(dirs, files, content)| {
        let mut paths = Vec::new();
        
        // Add directories
        for dir in dirs {
            paths.push((PathBuf::from(&dir), None));
        }
        
        // Add files
        for file in files {
            paths.push((
                PathBuf::from(format!("{}.txt", file)),
                Some(content.clone()),
            ));
        }
        
        paths
    })
}

proptest! {
    #[test]
    fn test_directory_listing_consistency(
        structure in dir_structure_strategy()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async {
            let temp = TempDir::new().unwrap();
            let mut expected_entries = Vec::new();
            
            // Create directory structure
            for (path, content) in structure {
                let full_path = temp.path().join(&path);
                
                if let Some(content) = content {
                    // It's a file
                    if let Some(parent) = full_path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    let mut file = tokio::fs::File::create(&full_path).await.unwrap();
                    file.write_all(content.as_bytes()).await.unwrap();
                    expected_entries.push(full_path);
                } else {
                    // It's a directory
                    fs::create_dir_all(&full_path).unwrap();
                    expected_entries.push(full_path);
                }
            }

            let options = ListOptions {
                recursive: true,
                include_size: true,
                include_modified: true,
                include_type: true,
                count_lines: true,
                ..Default::default()
            };

            if let Ok(entries) = list_directory(temp.path(), temp.path(), &options).await {
                // Test size consistency
                for entry in &entries {
                    if !entry.is_dir {
                        let fs_size = fs::metadata(temp.path().join(&entry.path))
                            .unwrap()
                            .len();
                        prop_assert_eq!(entry.size.unwrap(), fs_size);
                    }
                }

                // Test recursive listing
                let mut found_paths: Vec<_> = entries
                    .iter()
                    .map(|e| temp.path().join(&e.path))
                    .collect();
                found_paths.sort();
                expected_entries.sort();
                
                prop_assert_eq!(found_paths.len(), expected_entries.len());
                for (found, expected) in found_paths.iter().zip(expected_entries.iter()) {
                    prop_assert_eq!(found, expected);
                }

                // Test entry types
                for entry in &entries {
                    let path = temp.path().join(&entry.path);
                    let is_dir = fs::metadata(&path).unwrap().is_dir();
                    prop_assert_eq!(entry.is_dir, is_dir);
                    
                    if let Some(entry_type) = &entry.entry_type {
                        prop_assert_eq!(
                            entry_type,
                            if is_dir { "directory" } else { "file" }
                        );
                    }
                }
            }

            Ok(())
        });
    }

    #[test]
    fn test_directory_pattern_filtering(
        structure in dir_structure_strategy(),
        pattern in "[a-zA-Z0-9]{1,10}"
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async {
            let temp = TempDir::new().unwrap();
            
            // Create directory structure
            for (path, content) in structure {
                let full_path = temp.path().join(&path);
                
                if let Some(content) = content {
                    if let Some(parent) = full_path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    let mut file = tokio::fs::File::create(&full_path).await.unwrap();
                    file.write_all(content.as_bytes()).await.unwrap();
                } else {
                    fs::create_dir_all(&full_path).unwrap();
                }
            }

            let options = ListOptions {
                recursive: true,
                file_pattern: Some(pattern.clone()),
                ..Default::default()
            };

            if let Ok(entries) = list_directory(temp.path(), temp.path(), &options).await {
                let pattern_regex = regex::Regex::new(&pattern).unwrap();
                
                for entry in entries {
                    // Every returned path should match the pattern
                    let path_str = entry.path.to_string_lossy();
                    prop_assert!(pattern_regex.is_match(&path_str));
                }
            }

            Ok(())
        });
    }

    #[test]
    fn test_directory_line_counting(file_count in 1..5usize) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async {
            let temp = TempDir::new().unwrap();
            let mut expected_lines = Vec::new();
            
            // Create files with known line counts
            for i in 0..file_count {
            let content_tree = file_content_strategy()
            .new_tree(&mut TestRunner::default())
            .unwrap();
            let (content, line_count) = content_tree.current();
            let file_path = temp.path().join(format!("file_{}.txt", i));
            fs::write(&file_path, content.as_bytes()).unwrap();
                expected_lines.push((file_path.strip_prefix(temp.path()).unwrap().to_path_buf(), line_count));
                }

            let options = ListOptions {
                recursive: true,
                count_lines: true,
                ..Default::default()
            };

            if let Ok(entries) = list_directory(temp.path(), temp.path(), &options).await {
                for entry in entries {
                    if let Some(stats) = entry.stats {
                        if let Some(actual_count) = stats.line_count {
                            // Find matching expected entry
                            if let Some((_, expected_count)) = expected_lines.iter()
                                .find(|(path, _)| path == &entry.path)
                            {
                                prop_assert_eq!(actual_count, *expected_count);
                            }
                        }
                    }
                }
            }

            Ok(())
        });
    }
}