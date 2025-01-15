use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use tempfile::TempDir;
use tokio::fs;

use crate::fs_write::write_file;
use super::strategies::partial_write_strategy;

proptest_config! {
    ProptestConfig { 
        cases: 20,
        .. ProptestConfig::default()
    }
}

proptest! {
    #[test]
    fn test_partial_write_preserves_structure(data in partial_write_strategy()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _: Result<(), TestCaseError> = rt.block_on(async {
            let (content, operation) = data;
            
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");
            fs::write(&file_path, &content).await.unwrap();

            let result = write_file(&file_path, operation, None).await.unwrap();
            let final_content = fs::read_to_string(&file_path).await.unwrap();

            assert_eq!(result.line_count, final_content.lines().count());
            assert_eq!(result.size, final_content.len());

            Ok(())
        });
    }
}