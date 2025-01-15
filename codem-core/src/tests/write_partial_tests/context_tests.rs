use proptest::prelude::*;
use tempfile::TempDir;
use tokio::fs;
use std::ops::Range;

use crate::types::{PartialWrite, WriteOperation, Change};
use crate::fs_write::write_file;
use super::strategies::content_strategy;

proptest! {
    #[test]
    fn test_partial_write_context_lines(initial_content in content_strategy(), pattern in "[a-zA-Z0-9 ]{1,10}\n") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Run only 20 cases per test
            for context_lines in 0..2 {
                let dir = TempDir::new().unwrap();
                let file_path = dir.path().join("test.txt");

                let content = initial_content.clone() + &pattern;
                fs::write(&file_path, &content).await.unwrap();

                let operation = WriteOperation::Partial(PartialWrite {
                    context_lines,
                    return_full_content: true,
                    changes: vec![Change {
                        old_str: pattern.clone(),
                        new_str: "replacement\n".to_string(),
                        allow_multiple_matches: false,
                    }],
                });

                let result = write_file(&file_path, operation, None).await.unwrap();
                
                if let Some(partial_result) = result.partial_write_result {
                    for change in partial_result.change_results {
                        let context_line_count = change.context.lines().count();
                        let expected_range = Range {
                            start: 1,
                            end: 1 + 2 * context_lines + 1,
                        };
                        prop_assert!(expected_range.contains(&context_line_count));
                    }
                }
            }
            Ok(())
        }).unwrap();
    }
}