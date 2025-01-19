use tempfile::TempDir;
use std::fs;
use crate::error::ClientError;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_write_nonexistent_session() {
    let temp_dir = TempDir::new().unwrap();
    let client = create_test_client(temp_dir.path(), None).await;
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "initial content").unwrap();

    let result = client
        .write_file_full(
            "invalid-session",
            &file_path,
            "new",
            false
        )
        .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::SessionNotFound { .. }));
}