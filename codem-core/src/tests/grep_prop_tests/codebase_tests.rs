use proptest::prelude::*;
use tempfile::TempDir;
use tokio::fs;
use super::strategies::codebase_strategy;
use crate::types::GrepOptions;
use crate::grep::grep_codebase;
use regex::RegexBuilder;

proptest! {
    #[test]
    fn test_grep_codebase_with_varied_structure(structure in codebase_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let _ = fs::create_dir_all(dir.path()).await;
            
            for (path, content) in structure {
                let full_path = dir.path().join(path);
                if let Some(parent) = full_path.parent() {
                    let _ = fs::create_dir_all(parent).await;
                }
                let _ = fs::write(&full_path, content).await;
            }

            let pattern = RegexBuilder::new("test").build().unwrap();
            let options = GrepOptions::default();
            let _ = grep_codebase(dir.path(), &pattern, options).await;

            Ok::<(), TestCaseError>(())
        }).unwrap();
    }

    #[test]
    fn test_grep_file_pattern_variations(structure in codebase_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let _ = fs::create_dir_all(dir.path()).await;
            
            for (path, content) in &structure {
                let full_path = dir.path().join(path);
                if let Some(parent) = full_path.parent() {
                    let _ = fs::create_dir_all(parent).await;
                }
                let _ = fs::write(&full_path, content).await;
            }

            let pattern = RegexBuilder::new("test").build().unwrap();
            let file_patterns = ["*.txt"];
            
            for file_pattern in file_patterns {
                let options = GrepOptions {
                    file_pattern: Some(file_pattern.to_string()),
                    ..Default::default()
                };
                let _ = grep_codebase(dir.path(), &pattern, options).await;
            }
            
            Ok::<(), TestCaseError>(())
        }).unwrap();
    }
}