use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation, WriteResultDetails};
use crate::fs_write::write_file;
use tempfile::TempDir;
use rstest::rstest;

#[rstest]
#[case("start", "end", "Before\nstart\nContent\nend\nAfter\n", 1)]
#[tokio::test]
async fn test_basic_large_write(
    #[case] start_pattern: &str,
    #[case] end_pattern: &str,
    #[case] content: &str,
    #[case] context_lines: usize,
) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: start_pattern.to_string(),
        end_str: end_pattern.to_string(),
        new_str: "replaced".to_string(),
        context_lines,
        line_range: None,
    });

    let result = write_file(&file_path, operation, None).await.unwrap();

    if let WriteResultDetails::PartialLarge(write_result) = result.details {
        assert_eq!(write_result.line_number_start, 2);
        assert_eq!(write_result.line_number_end, 4);

        assert_eq!(write_result.context.before_start, vec!["Before"]);
        assert_eq!(write_result.context.start_content, vec!["replaced"]);
        assert_eq!(write_result.context.end_content, vec!["replaced"]);
        assert_eq!(write_result.context.after_end, vec!["After"]);
    }
}