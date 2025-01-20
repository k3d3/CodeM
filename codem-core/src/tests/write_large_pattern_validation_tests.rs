use tokio::fs;
use crate::types::{PartialWriteLarge, WriteOperation};
use crate::WriteError;
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_overlapping_patterns() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let test_cases = vec![
        ("AZ", "Z"), // End pattern contained in start 
        ("Z", "AZ"), // Start pattern contained in end
        ("ABC", "BC"), // Overlapping patterns
        ("CD", "BCD"), // Overlapping patterns
        ("Pattern", "Pattern1"), // One extends the other
    ];

    for (start, end) in test_cases {
        // Order doesn't matter since patterns are invalid
        let content = format!("{}\nContent\n{}\n", start, end);
        fs::write(&file_path, &content).await.unwrap();

        let operation = WriteOperation::PartialLarge(PartialWriteLarge {
            start_str: format!("{}\n", start),
            end_str: format!("{}\n", end),
            new_str: "new\n".to_string(),
            context_lines: 1,
            line_range: None,
        });

        let result = write_file(&file_path, operation, None).await;
        assert!(matches!(result, Err(WriteError::InvalidPatternPair { content: _ })));
    }
}

#[tokio::test]
async fn test_pattern_order() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // End pattern before start pattern
    let content = "\
END\n\
CONTENT\n\
START\n\
";
    fs::write(&file_path, &content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "new\n".to_string(),
        context_lines: 1,
        line_range: None,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::EndPatternBeforeStart { content: _ })));
}

#[tokio::test]
async fn test_duplicate_pattern() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    // Pattern appears twice
    let content = "\
START\n\
Content\n\
START\n\
END\n\
";
    fs::write(&file_path, &content).await.unwrap();

    let operation = WriteOperation::PartialLarge(PartialWriteLarge {
        start_str: "START\n".to_string(),
        end_str: "END\n".to_string(),
        new_str: "new\n".to_string(),
        context_lines: 1,
        line_range: None,
    });

    let result = write_file(&file_path, operation, None).await;
    assert!(matches!(result, Err(WriteError::MultipleStartPatternsFound { content: _ })));
}

#[tokio::test]
async fn test_valid_patterns() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let test_cases = vec![
        ("START", "END"),
        ("PATTERN_A", "PATTERN_B"),
        ("BEGIN_123", "END_123"),
    ];

    for (start, end) in test_cases {
        let content = format!("{}\nContent\n{}\n", start, end);
        fs::write(&file_path, &content).await.unwrap();

        let operation = WriteOperation::PartialLarge(PartialWriteLarge {
            start_str: format!("{}\n", start),
            end_str: format!("{}\n", end),
            new_str: "new\n".to_string(),
            context_lines: 1,
            line_range: None,
        });

        let result = write_file(&file_path, operation, None).await;
        assert!(result.is_ok());
    }
}