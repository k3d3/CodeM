use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, WriteResultDetails};
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_multiple_matches() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "Test\nMiddle\nTest\n";
    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "Test".to_string(),
            new_str: "New".to_string(),
            allow_multiple_matches: true,
        }],
    });

    let result = write_file(&file_path, operation, None).await.unwrap();

    if let WriteResultDetails::Partial(partial_result) = result.details {
        assert_eq!(partial_result.change_results.len(), 2);
        if let Some(written_content) = partial_result.full_content {
            assert_eq!(written_content.trim_end(), "New\nMiddle\nNew");
        }
    }
}