use super::common::{create_test_env, create_test_files};
use crate::tools::read;
use jsonrpc_stdio_server::jsonrpc_core::Value;

#[tokio::test]
async fn test_read_multiple_files_success() {
    let (mcp, _temp_dir) = create_test_env("test_project").await;
    create_test_files(&_temp_dir, "test_project");

    let session_id = mcp.client.create_session("test_project").await.unwrap();
    let paths = vec![
        "file1.txt".to_string(),
        "file2.txt".to_string()
    ];

    let result = read::read_files(&mcp, &session_id, paths).await.unwrap();

    if let Value::Object(map) = result {
        let content = map.get("content").unwrap().as_array().unwrap();
        assert_eq!(content.len(), 2);

        // Check files content
        let files_msg = &content[0];
        assert_eq!(files_msg.get("type").unwrap().as_str().unwrap(), "text");
        let text = files_msg.get("text").unwrap().as_str().unwrap();
        assert!(text.contains("file1.txt"));
        assert!(text.contains("Content of file 1"));
        assert!(text.contains("file2.txt"));
        assert!(text.contains("Content of file 2"));

        // Check summary content
        let summary_msg = &content[1];
        assert_eq!(summary_msg.get("type").unwrap().as_str().unwrap(), "text");
        let summary_text = summary_msg.get("text").unwrap().as_str().unwrap();
        assert!(summary_text.contains("Total files: 2"));
        assert!(summary_text.contains("Successful reads: 2"));
        assert!(summary_text.contains("Failed reads: 0"));
    } else {
        panic!("Expected object result");
    }
}