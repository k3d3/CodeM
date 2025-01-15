use rstest::rstest;
use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::WriteError;
use crate::fs_write::write_file;
use tempfile::TempDir;

#[rstest]
#[case("START", "END", "CONTENT")] // Basic non-overlapping case
#[tokio::test]
async fn test_valid_patterns(#[case] start: &str, #[case] end: &str, #[case] content: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("{}\n{}\n{}\n", start, content, end);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", start),
        end_str: format!("{}\n", end),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    // Should succeed
    let result = write_file(&file_path, operation, None).await;
    assert!(result.is_ok());
}

#[rstest]
#[case("END", "START", "CONTENT")] // End before start
#[tokio::test]
async fn test_invalid_order(#[case] start: &str, #[case] end: &str, #[case] content: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("{}\n{}\n{}\n", end, content, start);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", start),
        end_str: format!("{}\n", end),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    // Should return an error
    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::EndPatternBeforeStart)));
}

#[rstest]
#[case("AZ", "Z")] // Nested pattern
#[case("ABC", "BC")] // Overlapping patterns
#[tokio::test]
async fn test_invalid_pattern_pairs(#[case] start: &str, #[case] end: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // Order doesn't matter for this test since the patterns themselves are invalid
    let file_content = format!("{}\nMIDDLE\n{}\n", start, end);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", start),
        end_str: format!("{}\n", end),
        new_str: "new\n".to_string(),
        context_lines: 1,
    });

    // Should error due to invalid pattern pairs
    let result = write_file(&file_path, operation, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_content_preservation() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = "PREFIX\nSTART\nMIDDLE\nEND\nSUFFIX\n";
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "REPLACED\n".to_string(),
        context_lines: 1,
    });

    let _result = write_file(&file_path, operation, None).await.unwrap();

    // Verify final content
    let final_content = fs::read_to_string(&file_path).await.unwrap();

    // Non-matching content preserved
    assert!(final_content.starts_with("PREFIX\n"));
    assert!(final_content.ends_with("SUFFIX\n"));

    // New content present
    assert!(final_content.contains("REPLACED\n"));
}