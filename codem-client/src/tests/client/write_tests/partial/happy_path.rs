use crate::{
    types::file_ops::{WriteOperation, WriteOptions},
    Client,
};
use codem_core::types::PartialWrite;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_partial_write() {
    // Setup temp directory and file
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "original content").unwrap();

    // Create config
    let config_path = temp_dir.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp_dir.path().display()
    );
    fs::write(&config_path, &config).unwrap();

    // Create client and session
    let client = Client::new(&config_path).await.unwrap();
    let session_id = client.run_on_project("test").await.unwrap();

    // Read first to cache timestamp
    let _ = client.read(&session_id, &file_path).await.unwrap();

    // Create partial write operation
    let write = PartialWrite {
        pattern: "original".to_string(),
        replacement: "modified".to_string(),
        context_lines: 3,
    };

    let result = client
        .write_file(
            &session_id,
            &file_path,
            WriteOperation::Partial(write),
            WriteOptions::default(),
        )
        .await
        .unwrap();

    assert!(result.content.contains("modified content"));
    assert!(result.check_results.is_empty());
}