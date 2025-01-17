use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use tempfile::TempDir;
use std::fs;
use tokio::io::AsyncWriteExt;

use crate::directory::list_directory;
use crate::types::ListOptions;
use super::{strategies::dir_structure_strategy, utils::*};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn test_directory_listing_consistency(structure in dir_structure_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let temp = TempDir::new().unwrap();
            let mut expected_entries = vec![temp.path().to_path_buf()];
            
            for (path, content) in structure {
                let full_path = temp.path().join(&path);
                if let Some(content) = content {
                    if let Some(parent) = full_path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    let mut file = tokio::fs::File::create(&full_path).await.unwrap();
                    file.write_all(content.as_bytes()).await.unwrap();
                    expected_entries.push(full_path);
                } else {
                    fs::create_dir_all(&full_path).unwrap();
                    expected_entries.push(full_path);
                }
            }

            let options = ListOptions {
                recursive: true,
                include_size: true,
                include_modified: true,
                count_lines: true,
                ..Default::default()
            };

            if let Ok(entries) = list_directory(temp.path(), temp.path(), &options).await {
                verify_sizes(&entries, temp.path());

                let mut found_paths = Vec::new();
                collect_paths(&entries, &mut found_paths);
                let mut found_paths: Vec<_> = found_paths.iter()
                    .map(|p| temp.path().join(p))
                    .collect();

                found_paths.sort();
                expected_entries.sort();
                
                prop_assert_eq!(found_paths.len(), expected_entries.len(), 
                    "Found paths: {:?}, Expected: {:?}", found_paths, expected_entries);
                
                for (found, expected) in found_paths.iter().zip(expected_entries.iter()) {
                    prop_assert_eq!(found, expected);
                }

                verify_entry_types(&entries, temp.path());
            }

            Ok(())
        });
    }
}