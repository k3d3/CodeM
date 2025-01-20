use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, WriteResultDetails};
use crate::fs_write::write_file;
use tempfile::TempDir;
use rstest::rstest;

#[rstest]
#[case("Before\nTest\nAfter\n", "Test", "New", 1, "Before\nNew\nAfter")]
#[case("Before\nTest\nAfter\n", "Test", "New", 0, "New")]
#[case("Before\nTest\nMiddle\nAfter\n", "Test", "New", 2, "Before\nNew\nMiddle\nAfter")]
#[case("\nTest\n\n", "Test", "New", 1, "\nNew\n")]
#[case("Test\n", "Test", "New", 1, "New")]
#[case("Before\nTest\n", "Test", "New", 1, "Before\nNew")]
#[tokio::test]
async fn test_partial_write_context(
    #[case] initial_content: &str,
    #[case] pattern: &str,
    #[case] replacement: &str,
    #[case] context_lines: usize,
    #[case] expected_context: &str,
) {
    println!("Running test with:");
    println!("  initial_content: {:?}", initial_content);
    println!("  pattern: {:?}", pattern);
    println!("  replacement: {:?}", replacement);
    println!("  context_lines: {}", context_lines);
    println!("  expected_context: {:?}", expected_context);

    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, initial_content).await.unwrap();

    let operation = WriteOperation::Partial(PartialWrite {
        context_lines,
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

    // Verify line numbers
    if let WriteResultDetails::Partial(partial_result) = result.details {
        assert_eq!(partial_result.change_results.len(), 1);
        let change = &partial_result.change_results[0];
        
        // For single-line files, line number should be 1
        let expected_line = if initial_content.lines().count() == 1 { 1 } else { 2 };
        assert_eq!(change.line_number_start, expected_line);
        assert_eq!(change.line_number_end, expected_line);

        // Print actual context
        println!("\nActual context: {:?}", change.context);
        println!("Expected context: {:?}", expected_context);

        // Compare with expected context
        assert_eq!(change.context, expected_context);
    }
}