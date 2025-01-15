use crate::{
    types::file_ops::{WriteOperation, WriteOptions},
    error::{ClientError, SessionError},
    session::SessionId,
    Client,
};
use codem_core::types::PartialWrite;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_invalid_session() {
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
    let invalid_session = SessionId::new();

    let write = PartialWrite {
        pattern: "test".to_string(),
        replacement: "new".to_string(),
        context_lines: 3,
    };

    let result = client
        .write_file(
            &invalid_session,
            &file_path,
            WriteOperation::Partial(write),
            WriteOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(ClientError::SessionError(SessionError::SessionNotFound { .. }))));
}