use super::common::create_test_env;
use crate::tools::read;
use jsonrpc_stdio_server::jsonrpc_core::Value;

#[tokio::test]
async fn test_read_multiple_files_no_paths() {
    let (mcp, _temp_dir) = create_test_env("test_project");

    let session_id = mcp.client.create_session("test_project").await.unwrap();
    let paths: Vec<String> = vec![];

    let result = read::read_files(&mcp, &session_id, paths).await.unwrap();

    if let Value::Object(map) = result {
        let content = map.get("content").unwrap().as_array().unwrap();
        assert_eq!(content.len(), 2);

        // Check files content
        let files_msg = &content[0];
        assert_eq!(files_msg.get("type").unwrap().as_str().unwrap(), "text");
        let text = files_msg.get("text").unwrap().as_str().unwrap();
        assert!(text.contains("No files provided"));

        // Check summary content
        let summary_msg = &content[1];
        assert_eq!(summary_msg.get("type").unwrap().as_str().unwrap(), "text");
        let summary_text = summary_msg.get("text").unwrap().as_str().unwrap();
        assert!(summary_text.contains("Total files: 0"));
        assert!(summary_text.contains("Successful reads: 0"));
        assert!(summary_text.contains("Failed reads: 0"));
    } else {
        panic!("Expected object result");
    }
}