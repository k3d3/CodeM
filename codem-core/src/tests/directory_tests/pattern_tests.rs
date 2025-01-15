use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use tempfile::TempDir;
use std::fs;
use tokio::io::AsyncWriteExt;

use crate::directory::list_directory;
use crate::types::ListOptions;
use super::{strategies::dir_structure_strategy, utils::verify_pattern_matches};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn test_directory_pattern_filtering(
        structure in dir_structure_strategy(),
        base_pattern in "[a-zA-Z0-9]{1,10}"
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let temp = TempDir::new().unwrap();
            
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
                file_pattern: Some(base_pattern.clone()),
                ..Default::default()
            };

            if let Ok(entries) = list_directory(temp.path(), temp.path(), &options).await {
                let pattern_regex = regex::Regex::new(&base_pattern).unwrap();
                verify_pattern_matches(&entries, &pattern_regex);
            }

            Ok(())
        });
    }
}