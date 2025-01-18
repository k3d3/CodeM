use proptest::prelude::*;
use regex::Regex;
use std::fs;
use tempfile::TempDir;

use crate::grep::grep_codebase;
use crate::types::GrepOptions;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn test_grep_codebase_with_varied_structure(
        pattern in r"[a-zA-Z0-9]{1,10}", 
        max_depth in 1usize..5,
        max_files in 1usize..10,
        max_dirs in 1usize..5,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let temp = TempDir::new().unwrap();
            let dir = temp.path();
            
            // Create a nested directory structure
            for d in 0..max_dirs {
                let depth = d % max_depth + 1;
                let mut path = dir.to_path_buf();
                for i in 0..depth {
                    path = path.join(format!("dir_{}_{}", d, i));
                    fs::create_dir_all(&path).unwrap();
                }
                
                // Add some files in each directory
                for f in 0..max_files {
                    let file_path = path.join(format!("file_{}.txt", f));
                    fs::write(&file_path, "test content\n").unwrap();
                }
            }

            let regex = Regex::new(&pattern).unwrap();
            let options = GrepOptions::default();

            let _ = grep_codebase(&dir, &regex, &options).await;
        });
    }

    #[test]
    fn test_grep_file_pattern_variations(
        pattern in r"[a-zA-Z0-9]{1,10}",
        file_pattern in r"[a-zA-Z0-9.*]{1,10}",
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let temp = tempfile::tempdir().unwrap();
            let dir = temp.path();

            // Create a few test files
            for i in 1..=3 {
                let test_file = dir.join(format!("test{}.txt", i));
                fs::write(&test_file, "test content\n").unwrap();
            }

            let regex = Regex::new(&pattern).unwrap();
            let options = GrepOptions {
                file_pattern: Some(file_pattern),
                ..Default::default()
            };

            let _ = grep_codebase(&dir, &regex, &options).await;
        });
    }
}