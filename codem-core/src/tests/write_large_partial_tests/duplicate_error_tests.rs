use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::WriteError;
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_duplicate_patterns() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // Test multiple start patterns
    let content = "START\nMIDDLE\nSTART\nEND\n";
    fs::write(&file_path, content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::MultipleStartPatternsFound { content: _ })));

    // Test multiple end patterns
    let content = "START\nEND\nMIDDLE\nEND\n";
    fs::write(&file_path, content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::MultipleEndPatternsFound { content: _ })));
}