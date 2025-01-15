use tokio::fs;
use crate::types::WriteOperation;
use crate::fs_write::write_file;
use tempfile::TempDir;
use rstest::rstest;

#[rstest]
#[case("Test content\n", "New content\n")]
#[case("Test\ncontent\n", "New\ncontent\n")]
#[case("\n\n\n", "\n\n")]
#[tokio::test]
async fn test_write_file_full(#[case] initial_content: &str, #[case] new_content: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Full(new_content.to_string());
    let result = write_file(&file_path, operation, None).await.unwrap();

    let final_content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(final_content, new_content);
    assert_eq!(result.line_count, new_content.lines().count());
    assert_eq!(result.size, new_content.len());
}