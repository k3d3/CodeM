use crate::{
    types::file_ops::{WriteOperation, WriteOptions},
    error::{ClientError, FileError},
    Client,
};
use codem_core::types::PartialWrite;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_disallowed_path() {
    let temp_dir = TempDir::new().unwrap();
    let other_dir = TempDir::new().unwrap();
    let disallowed_file = other_dir.path().join("test.txt");
    fs::write(&disallowed_file, "test content").unwrap();

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

    let write = PartialWrite {
        pattern: "test".to_string(),
        replacement: "new".to_string(),
        context_lines: 3,
    };

    let result = client
        .write_file(
            &session_id,
            &disallowed_file,
            WriteOperation::Partial(write),
            WriteOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(ClientError::FileError(FileError::PathNotAllowed { .. }))));
}