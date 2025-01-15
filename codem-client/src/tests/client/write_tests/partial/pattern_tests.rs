use crate::{
    types::file_ops::{WriteOperation, WriteOptions},
    error::{ClientError, FileError},
    Client,
};
use codem_core::types::PartialWrite;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_pattern_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test content").unwrap();

    let config_path = temp_dir.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp_dir.path().display()
    );
    fs::write(&config_path, &config).unwrap();

    let client = Client::new(&config_path).await.unwrap();
    let session_id = client.create_session("test").await.unwrap();
    let _ = client.read(&session_id, &file_path).await.unwrap();

    let write = PartialWrite {
        pattern: "nonexistent".to_string(),
        replacement: "new".to_string(),
        context_lines: 3,
    };

    let result = client
        .write_file(
            &session_id,
            &file_path,
            WriteOperation::Partial(write),
            WriteOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(ClientError::FileError(FileError::PatternNotFound { .. }))));
}