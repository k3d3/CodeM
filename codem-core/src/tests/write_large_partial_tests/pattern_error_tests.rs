use rstest::rstest;
use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::WriteError;
use crate::fs_write::write_file;
use tempfile::TempDir;

#[rstest]
#[case("END", "START")]
#[case("FINISH", "BEGIN")]
#[tokio::test]
async fn test_invalid_order(#[case] first: &str, #[case] second: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("{}\nMIDDLE\n{}\n", first, second);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", second),
        end_str: format!("{}\n", first),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::EndPatternBeforeStart { content: _ })));
}

#[rstest]
#[case("AZ", "Z")]
#[case("Z", "AZ")] 
#[case("ABC", "BC")]
#[case("Pattern", "Pattern1")]
#[tokio::test]
async fn test_overlapping_patterns(#[case] start: &str, #[case] end: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("{}\nContent\n{}\n", start, end);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", start),
        end_str: format!("{}\n", end),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::InvalidPatternPair { content: _ })));
}