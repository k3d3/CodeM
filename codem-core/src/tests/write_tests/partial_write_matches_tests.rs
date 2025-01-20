use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, WriteResultDetails};
use crate::fs_write::write_file;
use tempfile::TempDir;
use rstest::rstest;

#[rstest]
#[case("Test\n", "Test", "New", 1, "New")]
#[tokio::test]
async fn test_partial_write_matches(
    #[case] initial_content: &str,
    #[case] pattern: &str,
    #[case] replacement: &str,
    #[case] expected_count: usize,
    #[case] expected_content: &str,
) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        line_range: None,
        changes: vec![Change {
            old_str: pattern.to_string(),
            new_str: replacement.to_string(),
            allow_multiple_matches: false,
            line_range: None,
        }],
    });

    let result = write_file(&file_path, operation, None).await.unwrap();

    if let WriteResultDetails::Partial(partial_result) = result.details {
        assert_eq!(partial_result.change_results.len(), expected_count);
        if let Some(written_content) = partial_result.full_content {
            assert_eq!(written_content.trim_end(), expected_content);
        }
    }
}