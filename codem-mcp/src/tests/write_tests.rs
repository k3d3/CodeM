use std::path::PathBuf;
use jsonrpc_stdio_server::jsonrpc_core::Value;
use serde_json::json;
use crate::{server::Mcp, tools::write::functions::write_file_small};
use codem_core::types::{Change, LineRange};
use rstest::rstest;

#[rstest]
#[tokio::test]
async fn test_write_file_small_with_line_range() -> anyhow::Result<()> {
    let test_content = r#"Line 1 has a pattern
Line 2 has a pattern 
Line 3 has something else
Line 4 has a pattern
Line 5 has a pattern"#;

    let mcp = Mcp::new_test()?;
    let session_id = mcp.create_test_session()?;
    let path = "test.txt";

    // Write the test file
    mcp.write_test_file(path, test_content)?;

    // Test line range restriction
    let changes = vec![Change {
        old_str: "pattern".to_string(),
        new_str: "PATTERN".to_string(),
        allow_multiple_matches: false,
        line_range: Some(LineRange {
            start: Some(2),
            end: Some(4),
        }),
    }];

    let result = write_file_small(&mcp, &session_id, path, changes, false).await?;

    // The write should succeed
    if let Value::Object(obj) = result {
        let content = obj.get("content").and_then(|c| c.as_array()).unwrap();
        let text = content[0].get("text").and_then(|t| t.as_str()).unwrap();
        assert!(text.contains("File updated successfully"));
        
        // Verify file contents
        let result = mcp.read_file(&session_id, &PathBuf::from(path))?;
        let expected = r#"Line 1 has a pattern
Line 2 has a PATTERN 
Line 3 has something else
Line 4 has a PATTERN
Line 5 has a pattern"#;
        assert_eq!(result, expected);
    } else {
        panic!("Expected object response");
    }

    Ok(())
}