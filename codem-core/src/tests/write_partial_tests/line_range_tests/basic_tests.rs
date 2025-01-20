use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, WriteResultDetails, LineRange};
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_single_line_range() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "prefix\nold\ncontent\nold\nsuffix\n";
    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "old".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: true,
            line_range: Some(LineRange {
                start: Some(1),
                end: Some(2),
            }),
        }],
    });

    let result = write_file(&file_path, operation, None).await.unwrap();

    if let WriteResultDetails::Partial(partial_result) = result.details {
        assert_eq!(partial_result.change_results.len(), 1);
        if let Some(written_content) = partial_result.full_content {
            assert_eq!(written_content.trim_end(), "prefix\nnew\ncontent\nold\nsuffix");
        }
    }
}

#[tokio::test]
async fn test_multiple_matches_within_range() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "line 1 old\nline 2 old\nline 3 skip\nline 4 old\nline 5 old\n";
    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "old".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: true,
            line_range: Some(LineRange {
                start: Some(2),
                end: Some(4),
            }),
        }],
    });

    let result = write_file(&file_path, operation, None).await.unwrap();

    if let WriteResultDetails::Partial(partial_result) = result.details {
        assert_eq!(partial_result.change_results.len(), 2, "Should have replaced 2 occurrences within range");
        if let Some(written_content) = partial_result.full_content {
            assert_eq!(written_content.trim_end(), "line 1 old\nline 2 new\nline 3 skip\nline 4 new\nline 5 old");
        }
    }
}