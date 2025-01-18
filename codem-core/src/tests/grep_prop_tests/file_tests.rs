use proptest::prelude::*;
use regex::Regex;
use tempfile::TempDir;
use tokio::io::AsyncWriteExt;

use crate::grep::grep_codebase;
use crate::types::GrepOptions;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn test_grep_file_no_crashes(
        content in r"[a-zA-Z0-9\s\n]{0,1000}",
        pattern in r"[a-zA-Z0-9]{1,10}",
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Create a temporary file with the content
            let dir = TempDir::new().unwrap();
            let file = dir.path().join("test.txt");
            let mut f = tokio::fs::File::create(&file).await.unwrap();
            f.write_all(content.as_bytes()).await.unwrap();

            let regex = Regex::new(&pattern).unwrap();
            let options = GrepOptions::default();

            let _ = grep_codebase(dir.path(), &regex, &options).await;
        });
    }

    #[test]
    fn test_grep_file_with_varying_contexts(
        context_lines in 0usize..10,
        content in r"[a-zA-Z0-9\s\n]{0,1000}",
        pattern in r"[a-zA-Z0-9]{1,10}",
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file = dir.path().join("test.txt");
            let mut f = tokio::fs::File::create(&file).await.unwrap();
            f.write_all(content.as_bytes()).await.unwrap();

            let regex = Regex::new(&pattern).unwrap();
            let options = GrepOptions {
                context_lines,
                ..Default::default()
            };

            if let Ok(result) = grep_codebase(dir.path(), &regex, &options).await {
                if let Some(file_match) = result.first() {
                    for grep_match in &file_match.matches {
                        let context_lines = grep_match.context.lines().count();
                        // Context includes the match line plus context lines before and after
                        assert!(context_lines <= 1 + 2 * options.context_lines,
                            "Context should not exceed match line plus context lines");
                    }
                }
            }
        });
    }
}