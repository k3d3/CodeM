use rstest::rstest;
use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation, LineRange};
use crate::WriteError;
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_marker_handling() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // Test that both markers get excluded
    let file_content = "before\n<start>middle\nline2\n<end>\nafter\n";
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "<start>".to_string(),
        end_str: "<end>".to_string(),
        new_str: "replaced\n".to_string(),
        context_lines: 1,
        line_range: None,
    });

    let _result = write_file(&file_path, operation, None).await.unwrap();
    let final_content = fs::read_to_string(&file_path).await.unwrap();

    assert_eq!(final_content, "before\nreplaced\nafter\n");
}

#[rstest]
#[case("  START  ", "  END  ", "new\n")] // Whitespace trimming
#[case("START\n", "END\n", "new\n")] // Newlines in patterns
#[tokio::test]
async fn test_pattern_validation(#[case] start: &str, #[case] end: &str, #[case] new_content: &str) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let file_content = format!("before\n{}middle\n{}\nafter\n", start, end);
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: start.to_string(),
        end_str: end.to_string(),
        new_str: new_content.to_string(),
        context_lines: 1,
        line_range: None,
    });

    let _result = write_file(&file_path, operation, None).await.unwrap();
    let final_content = fs::read_to_string(&file_path).await.unwrap();
    assert!(final_content.contains(&new_content));
}

#[tokio::test]
async fn test_line_range_restriction() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // Test complex case with identical markers at different line numbers
    let file_content = "line1\nline2<START>replacement1<END>line2b\nline3\nline4<START>replacement2<END>line4b\nline5\n";
    fs::write(&file_path, &file_content).await.unwrap();

    // Replace only the second occurrence using line range
    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "<START>".to_string(),
        end_str: "<END>".to_string(),
        new_str: "REPLACED".to_string(),
        context_lines: 1,
        line_range: Some(LineRange {
            start: Some(4),  // Only match patterns containing line 4
            end: Some(4),
        }),
    });

    let _result = write_file(&file_path, operation, None).await.unwrap();
    let final_content = fs::read_to_string(&file_path).await.unwrap();

    // First marker pair should remain unchanged
    assert!(final_content.contains("<START>replacement1<END>"));
    // Second marker pair should be replaced
    assert_eq!(final_content, "line1\nline2<START>replacement1<END>line2b\nline3\nline4REPLACEDline4b\nline5\n");
}

#[tokio::test]
async fn test_line_range_with_newlines() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // Test case with newlines in markers and content
    let file_content = "line1\nSTART\ninner1\ninner2\nEND\nline3\nSTART\nreplace me\nEND\nline5\n";
    fs::write(&file_path, &file_content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "REPLACED\n".to_string(),
        context_lines: 1,
        line_range: Some(LineRange {
            start: Some(7),  // Target second block
            end: Some(9),
        }),
    });

    let _result = write_file(&file_path, operation, None).await.unwrap();
    let final_content = fs::read_to_string(&file_path).await.unwrap();

    // First block should be unchanged
    assert!(final_content.contains("START\ninner1\ninner2\nEND\n"));
    // Second block should be replaced with proper newline handling
    assert!(final_content.contains("REPLACED\n"));
}