use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, LineRange, PartialWriteLarge};
use crate::fs_write::write_file;
use crate::WriteError;
use tempfile::TempDir;

#[tokio::test]
async fn test_pattern_not_found_in_range() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "old\nold\nold\nold\n";
    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "notfound".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: Some(1),
                end: Some(2),
            }),
        }],
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::PatternNotFound { .. })));
}

#[tokio::test]
async fn test_large_write_with_range() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "prefix\nSTART\nA\nEND\nSTART\nB\nEND\nsuffix\n";
    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "CHANGED\n".to_string(),
        context_lines: 0,
        line_range: Some(LineRange {
            start: Some(5),
            end: Some(7),
        }),
    });

    let result = write_file(&file_path, operation, None).await.unwrap();
    let final_content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(final_content, "prefix\nSTART\nA\nEND\nSTART\nCHANGED\nEND\nsuffix\n");
}