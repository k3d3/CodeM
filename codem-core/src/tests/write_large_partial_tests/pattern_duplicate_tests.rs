use rstest::rstest;
use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::WriteError;
use crate::fs_write::write_file;
use tempfile::TempDir;

#[rstest]
#[case("START", "END")]
#[case("S", "E")]
#[case("A_START", "B_END")]
#[tokio::test]
async fn test_simple_pattern_order(#[case] start: &str, #[case] end: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // End pattern appears before start pattern
    let file_content = format!("{}\nCONTENT\n{}\n", end, start);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", start),
        end_str: format!("{}\n", end),
        new_str: "new\n".to_string(),
        context_lines: 1,
        line_range: None,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::EndPatternBeforeStart { content: _ })));

    let final_content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(file_content, final_content);
}

#[rstest]
#[case("PATTERN")] 
#[case("START")]
#[tokio::test]
async fn test_duplicate_patterns(#[case] pattern: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("{0}\nMIDDLE\n{0}\n", pattern);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: format!("{}\n", pattern),
        end_str: format!("{}\n", pattern),
        new_str: "new\n".to_string(),
        context_lines: 1,
        line_range: None,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::MultipleStartPatternsFound { content: _ })));
}