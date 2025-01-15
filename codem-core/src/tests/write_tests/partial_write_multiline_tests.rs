use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change};
use crate::fs_write::write_file;
use tempfile::TempDir;
use rstest::rstest;

#[rstest]
#[case("Test\nline\n", "Test\nline\n", "New\nstring\n")]
#[case("Before\nTest\nline\nAfter\n", "Test\nline\n", "New\nstring\n")]
#[case("Test\nline\nAfter\n", "Test\nline\n", "New\nstring\n")]
#[tokio::test]
async fn test_partial_write_multiline_replacements(
    #[case] initial_content: &str,
    #[case] pattern: &str,
    #[case] replacement: &str,
) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines: 1,
        return_full_content: true,
        changes: vec![Change {
            old_str: pattern.to_string(),
            new_str: replacement.to_string(),
            allow_multiple_matches: false,
        }],
    });

    let result = write_file(&file_path, operation, None).await.unwrap();
    if let Some(partial_result) = result.partial_write_result {
        assert_eq!(partial_result.change_results.len(), 1);
    }
}