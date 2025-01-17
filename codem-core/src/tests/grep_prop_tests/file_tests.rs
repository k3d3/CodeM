use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use tempfile::TempDir;
use tokio::fs;
use super::strategies::file_content_strategy;
use crate::types::GrepOptions;
use crate::grep::grep_file;
use regex::RegexBuilder;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn test_grep_file_no_crashes(content in file_content_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");
            fs::write(&file_path, &content).await.unwrap();

            let pattern = RegexBuilder::new("test").build().unwrap();
            let options = GrepOptions::default();

            let _ = grep_file(&file_path, &pattern, &options).await;
            Ok(())
        });
    }

    #[test]
    fn test_grep_file_with_varying_contexts(content in file_content_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");
            fs::write(&file_path, &content).await.unwrap();

            let pattern = RegexBuilder::new("test").build().unwrap();
            for context_lines in 0..=5 {
                let options = GrepOptions {
                    context_lines,
                    ..Default::default()
                };
                let _ = grep_file(&file_path, &pattern, &options).await;
            }
            Ok(())
        });
    }
}