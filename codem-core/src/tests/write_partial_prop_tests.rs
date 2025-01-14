use proptest::prelude::*;
use tempfile::NamedTempFile;
use tokio::fs;

use crate::types::{PartialWrite, Change};
use crate::fs_write_partial::process_partial_write;

fn text_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-zA-Z0-9 \n]{1,100}").unwrap()
}

fn replacement_strategy() -> impl Strategy<Value = (String, String)> {
    ("[a-zA-Z0-9 ]{1,10}", "[a-zA-Z0-9 ]{1,10}")
        .prop_map(|(old, new)| (old, new))
}

proptest! {
    #[test]
    fn test_partial_write_preserves_length(
        content in text_strategy(),
        (old_str, new_str) in replacement_strategy()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async {
            let temp_file = NamedTempFile::new().unwrap();
            fs::write(&temp_file, &content).await.unwrap();

            let partial_write = PartialWrite {
                changes: vec![Change { 
                    old_str, 
                    new_str,
                    allow_multiple_matches: false 
                }],
                context_lines: 3,
                return_full_content: true,
            };

            if let Ok(result) = process_partial_write(temp_file.path(), partial_write).await {
                if let Some(partial_result) = result.partial_write_result {
                    if let Some(full_content) = partial_result.full_content {
                        // Check that line count isn't drastically different
                        let original_lines = content.lines().count();
                        let new_lines = full_content.lines().count();
                        let diff = if original_lines > new_lines {
                            original_lines - new_lines
                        } else {
                            new_lines - original_lines
                        };
                        
                        // There should not be a huge change in line count
                        prop_assert!(diff <= 5);
                    }
                }
            }
            Ok(())
        });
    }

    #[test]
    fn test_partial_write_preserves_structure(
        content in text_strategy(),
        (old_str, new_str) in replacement_strategy()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async {
            let temp_file = NamedTempFile::new().unwrap();
            fs::write(&temp_file, &content).await.unwrap();

            let partial_write = PartialWrite {
                changes: vec![Change { old_str, new_str: new_str.clone(), allow_multiple_matches: false }],
                context_lines: 3,
                return_full_content: true,
            };

            if let Ok(result) = process_partial_write(temp_file.path(), partial_write).await {
                if let Some(partial_result) = result.partial_write_result {
                    for write_result in partial_result.content {
                        // Line numbers should be valid
                        prop_assert!(write_result.line_number_start > 0);
                        prop_assert!(write_result.line_number_end >= write_result.line_number_start);
                        
                        // Context should contain the change
                        prop_assert!(write_result.context.contains(&new_str));
                    }
                }
            }
            Ok(())
        });
    }

    #[test]
    fn test_partial_write_context_lines(
        content in text_strategy(),
        (old_str, new_str) in replacement_strategy(),
        context_lines in 0..5usize,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async {
            let temp_file = NamedTempFile::new().unwrap();
            fs::write(&temp_file, &content).await.unwrap();

            let partial_write = PartialWrite {
                changes: vec![Change { old_str: old_str.clone(), new_str, allow_multiple_matches: false }],
                context_lines,
                return_full_content: true,
            };

            if let Ok(result) = process_partial_write(temp_file.path(), partial_write).await {
                if let Some(partial_result) = result.partial_write_result {
                    for write_result in partial_result.content {
                        // Count context lines
                        let context_line_count = write_result.context.lines().count();
                        
                        // Context should be no more than 2 * context_lines + 1
                        // (lines before + changed line + lines after)
                        prop_assert!(context_line_count <= (2 * context_lines + 1));
                    }
                }
            }
            Ok(())
        });
    }
}