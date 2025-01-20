use tokio::fs;
use crate::types::{PartialWrite, WriteOperation, Change, LineRange};
use crate::fs_write::write_file;
use tempfile::TempDir;

#[tokio::test]
async fn test_partial_write_vs_change_line_range() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "Line 1 pattern\nLine 2 pattern\nLine 3 other\nLine 4 pattern\nLine 5 pattern\n";
    fs::write(&file_path, initial_content).await.unwrap();

    // Case 1: Only change line range
    let operation1 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        line_range: None,
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: Some(2),
                end: Some(4),
            }),
        }],
    });

    // This should fail with multiple matches because even with line_range 2-4,
    // it finds a match on line 2 and line 4
    let result1 = write_file(&file_path, operation1, None).await;
    assert!(result1.is_err(), "Expected error due to multiple matches in change line range");

    // Case 2: Only partial write line range
    let operation2 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        line_range: Some(LineRange {
            start: Some(2),
            end: Some(4),
        }),
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: None,
        }],
    });

    // This should also fail with multiple matches because the overall line range 
    // still catches multiple occurrences
    let result2 = write_file(&file_path, operation2, None).await;
    assert!(result2.is_err(), "Expected error due to multiple matches in partial write line range");

    // Case 3: Both line ranges - intersecting to limit to just one match
    let operation3 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        line_range: Some(LineRange {
            start: Some(2),
            end: Some(3),
        }),
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: Some(2),
                end: Some(2),
            }),
        }],
    });

    // This should succeed because both line ranges together ensure only one match
    let result3 = write_file(&file_path, operation3, None).await;
    assert!(result3.is_ok(), "Expected success when line ranges intersect to single match");

    // Case 4: Both line ranges - non-intersecting
    let operation4 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        line_range: Some(LineRange {
            start: Some(1),
            end: Some(2),
        }),
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: Some(3),
                end: Some(4),
            }),
        }],
    });

    // This should fail with no matches because the line ranges don't intersect
    let result4 = write_file(&file_path, operation4, None).await;
    assert!(result4.is_err(), "Expected error when line ranges don't intersect");
}

#[tokio::test]
async fn test_line_range_edge_cases() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");

    let initial_content = "Line 1 pattern\nLine 2 pattern\nLine 3 pattern\n";
    fs::write(&file_path, initial_content).await.unwrap();

    // Case 1: Only start line specified
    let operation1 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: Some(2),
                end: None,
            }),
        }],
    });

    // Should fail because it matches lines 2 and 3
    let result1 = write_file(&file_path, operation1, None).await;
    assert!(result1.is_err(), "Expected error with only start line");

    // Case 2: Only end line specified
    let operation2 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: None,
                end: Some(2),
            }),
        }],
    });

    // Should fail because it matches lines 1 and 2
    let result2 = write_file(&file_path, operation2, None).await;
    assert!(result2.is_err(), "Expected error with only end line");

    // Case 3: Empty line range (should be treated as no range)
    let operation3 = WriteOperation::Partial(PartialWrite {
        context_lines: 0,
        return_full_content: true,
        changes: vec![Change {
            old_str: "pattern".to_string(),
            new_str: "new".to_string(),
            allow_multiple_matches: false,
            line_range: Some(LineRange {
                start: None,
                end: None,
            }),
        }],
    });

    // Should fail because it matches all lines
    let result3 = write_file(&file_path, operation3, None).await;
    assert!(result3.is_err(), "Expected error with empty line range");
}