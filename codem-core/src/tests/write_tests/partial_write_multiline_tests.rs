use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, WriteResultDetails};
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_partial_write_multiline() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "Before\nTest\nLine1\nLine2\nAfter\n";
    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "Test\nLine1\nLine2".to_string(),
            new_str: "New".to_string(),
            allow_multiple_matches: false,
        }],
    });

    let result = write_file(&file_path, operation, None).await.unwrap();

    if let WriteResultDetails::Partial(partial_result) = result.details {
        assert_eq!(partial_result.change_results.len(), 1);
        if let Some(content) = partial_result.full_content {
            assert_eq!(content.trim_end(), "Before\nNew\nAfter");
        }
    }
}