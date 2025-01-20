use tempfile::TempDir;
use std::fs;
use codem_core::types::Change;
use crate::error::ClientError;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_partial_multiple_matches() {
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir_all(temp_dir.path().join("session")).unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "line\nline\nline\n").unwrap();

    let client = create_test_client(temp_dir.path(), None).await;
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    let changes = vec![Change {
        new_str: "new\n".to_string(),
        old_str: "line\n".to_string(),
        allow_multiple_matches: false,
        line_range: None,
    }];

    let result = client
        .write_file_partial(
            &session_id,
            &file_path,
            changes,
            false
        )
        .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::WriteError(..)));
}