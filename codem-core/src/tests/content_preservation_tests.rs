use tokio::fs;
use proptest::prelude::*;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::fs_write::write_file;
use tempfile::TempDir;

proptest! {
    #[test]
    fn test_content_preservation(
        before in "[^\n]{0,50}",
        content in "[^\n]{0,50}",
        after in "[^\n]{0,50}",
        new_content in "[^\n]{0,50}"
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = TempDir::new().unwrap();
            let file_path = dir.path().join("test.txt");
            
            let file_content = format!("{}\nSTART\n{}\nEND\n{}\n", before, content, after);
            fs::write(&file_path, &file_content).await.unwrap();

            let operation = WriteOperation::PartialLarge(PartialWriteLarge {
                start_str: "START\n".to_string(),
                end_str: "END\n".to_string(),
                new_str: format!("{}\n", new_content),
                context_lines: 1,
            });

            let result = write_file(&file_path, operation, None).await.unwrap();
            let final_content = fs::read_to_string(&file_path).await.unwrap();

            // Before content preserved
            assert!(final_content.starts_with(&format!("{}\n", before)));
            // New content inserted
            assert!(final_content.contains(&format!("{}\n", new_content)));
            // After content preserved
            assert!(final_content.ends_with(&format!("{}\n", after)));
        });
    }
}