use rstest::rstest;
use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::fs_write::write_file;
use tempfile::TempDir;

#[rstest]
#[case("AZ", "Z")]        // Nested end in start
#[case("ABC", "BC")]      // Overlapping patterns
#[case("START_", "START")] // Prefix pattern
#[tokio::test]
async fn test_nested_pattern_handling(#[case] start: &str, #[case] end: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("{}\nCONTENT\n{}\n", start, end);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", start),
        end_str: format!("{}\n", end),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    let result = write_file(&file_path, operation, None).await;
    // These patterns should be rejected
    assert!(result.is_err());
}