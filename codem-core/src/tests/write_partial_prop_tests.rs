use proptest::prelude::*;
use crate::types::{PartialWrite, WriteOperation, Change, WriteResultDetails};
use tempfile::TempDir;
use crate::fs_write::write_file;
use tokio;

proptest! {
    #[test]
    fn doesnt_crash(
        content in "\\PC*",
        pattern in "\\PC+",
        replacement in "\\PC*",
        context_lines in 0usize..5,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");

            // Ensure content is not empty
            let content = if content.is_empty() { "default".to_string() } else { content };
            tokio::fs::write(&file_path, &content).await.unwrap();

            let operation = WriteOperation::Partial(PartialWrite {
                context_lines,
                return_full_content: true,
                changes: vec![Change {
                    old_str: pattern,
                    new_str: replacement,
                    allow_multiple_matches: false,
                }],
            });

            if let Ok(result) = write_file(&file_path, operation, None).await {
                if let WriteResultDetails::Partial(partial_result) = result.details {
                    if let Some(written_content) = partial_result.full_content {
                        assert!(!written_content.is_empty());
                    }
                }
            }
        });
    }

    #[test]
    fn preserves_content_around_matches(
        prefix in "[a-z]+",
        content in "[0-9]+",
        suffix in "[a-z]+",
        pattern in "[0-9]+",
        replacement in "[a-z]*",
        context_lines in 0usize..5
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");

            let full_content = format!("{}{}{}", prefix, content, suffix);
            tokio::fs::write(&file_path, &full_content).await.unwrap();

            let operation = WriteOperation::Partial(PartialWrite {
                context_lines,
                return_full_content: true,
                changes: vec![Change {
                    old_str: pattern,
                    new_str: replacement,
                    allow_multiple_matches: false,
                }],
            });

            if let Ok(result) = write_file(&file_path, operation, None).await {
                if let WriteResultDetails::Partial(partial_result) = result.details {
                    if let Some(written_content) = partial_result.full_content {
                        assert!(written_content.starts_with(&prefix));
                        assert!(written_content.ends_with(&suffix));
                    }
                }
            }
        });
    }

    #[test]
    fn maintains_line_count_with_no_matches(
        content in "\\PC*\\n\\PC*\\n\\PC*",
        pattern in "nomatch",
        replacement in "\\PC*",
        context_lines in 0usize..5
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");

            tokio::fs::write(&file_path, &content).await.unwrap();
            let initial_line_count = content.lines().count();

            let operation = WriteOperation::Partial(PartialWrite {
                context_lines,
                return_full_content: true,
                changes: vec![Change {
                    old_str: pattern.to_string(),
                    new_str: replacement,
                    allow_multiple_matches: false,
                }],
            });

            if let Ok(result) = write_file(&file_path, operation, None).await {
                if let WriteResultDetails::Partial(partial_result) = result.details {
                    if let Some(written_content) = partial_result.full_content {
                        assert_eq!(written_content.lines().count(), initial_line_count);
                    }
                }
            }
        });
    }
}