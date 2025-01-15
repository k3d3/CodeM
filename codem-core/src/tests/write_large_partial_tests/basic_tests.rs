use rstest::rstest;
use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::fs_write::write_file;
use tempfile::TempDir;

#[rstest]
#[case("START", "END", "CONTENT")]
#[case("START_A", "END_B", "MIDDLE")]
#[case("A_START", "B_END", "TEXT")]
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

    let result = write_file(&file_path, operation, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_content_preservation() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let content = "PREFIX\nSTART\nMIDDLE\nEND\nSUFFIX\n";
    fs::write(&file_path, content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "REPLACED\n".to_string(),
        context_lines: 1,
    });

    let result = write_file(&file_path, operation, None).await.unwrap();
    let write_result = result.partial_write_large_result.unwrap();

    let final_content = fs::read_to_string(&file_path).await.unwrap();
    
    // Original content preserved
    assert!(final_content.starts_with("PREFIX\n"));
    assert!(final_content.ends_with("SUFFIX\n"));

    // New content present 
    assert!(final_content.contains("REPLACED\n"));

    // Context lines correct
    assert!(write_result.context.before_start.len() <= 1);
    assert!(write_result.context.after_end.len() <= 1);
    assert!(write_result.context.start_content.len() <= 2);
    assert!(write_result.context.end_content.len() <= 2);
}