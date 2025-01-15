use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use tempfile::TempDir;
use tokio::fs;
use super::strategies::codebase_strategy;
use crate::types::GrepOptions;
use crate::grep::grep_codebase;
use regex::RegexBuilder;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn test_grep_codebase_with_varied_structure(structure in codebase_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let dir = TempDir::new().unwrap();
            
            // Create codebase structure
            for (path, content) in structure {
                let full_path = dir.path().join(path);
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent).await.unwrap();
                }
                fs::write(&full_path, content).await.unwrap();
            }

            // Test grep with various patterns
            let pattern = RegexBuilder::new("test").build().unwrap();
            let options = GrepOptions::default();
            let _ = grep_codebase(dir.path(), &pattern, options).await;

            Ok(())
        });
    }

    #[test]
    fn test_grep_file_pattern_variations(structure in codebase_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let dir = TempDir::new().unwrap();
            
            // Create codebase structure
            for (path, content) in structure {
                let full_path = dir.path().join(path);
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent).await.unwrap();
                }
                fs::write(&full_path, content).await.unwrap();
            }

            // Test with various file patterns
            let pattern = RegexBuilder::new("test").build().unwrap();
            let file_patterns = vec!["*.txt", "test*", "*.rs"];
            
            for file_pattern in file_patterns {
                let options = GrepOptions {
                    file_pattern: Some(file_pattern.to_string()),
                    ..Default::default()
                };
                let _ = grep_codebase(dir.path(), &pattern, options).await;
            }

            Ok(())
        });
    }
}