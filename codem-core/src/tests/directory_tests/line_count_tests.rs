use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{TestCaseError, TestRunner};
use tempfile::TempDir;
use std::fs;

use crate::directory::list_directory;
use crate::types::ListOptions;
use super::{strategies::file_content_strategy, utils::verify_line_counts};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn test_directory_line_counting(file_count in 1..5usize) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let temp = TempDir::new().unwrap();
            let mut expected_lines = Vec::new();
            
            for i in 0..file_count {
                let content_tree = file_content_strategy()
                    .new_tree(&mut TestRunner::default())
                    .unwrap();
                let (content, line_count) = content_tree.current();
                let file_path = temp.path().join(format!("file_{}.txt", i));
                fs::write(&file_path, content.as_bytes()).unwrap();
                expected_lines.push((
                    file_path.strip_prefix(temp.path()).unwrap().to_path_buf(),
                    line_count
                ));
            }

            let options = ListOptions {
                recursive: true,
                count_lines: true,
                ..Default::default()
            };

            if let Ok(entries) = list_directory(temp.path(), temp.path(), &options).await {
                verify_line_counts(&entries, &expected_lines);
            }

            Ok(())
        });
    }
}