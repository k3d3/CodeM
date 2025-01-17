use tempfile::TempDir;
use std::fs;
use crate::error::ClientError;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_write_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let client = create_test_client(temp_dir.path(), None);
    
    // Create session and let it initialize
    let session_id = client.create_session("test").await.unwrap();
    
    // Create a file and get its timestamp to ensure session is properly initialized
    let init_file = temp_dir.path().join("init.txt");
    fs::write(&init_file, "initial").unwrap();
    client.read_file(&session_id, &init_file).await.unwrap();

    // Now try nonexistent file
    let file_path = temp_dir.path().join("nonexistent.txt");

    let result = client
        .write_file_full(
            &session_id,
            &file_path,
            "new",
            false
        )
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        ClientError::FileNotFound { .. } => (), // Expected error
        err => panic!("Expected FileNotFound, got {:?}", err),
    }
}

#[tokio::test]
async fn test_write_readable_not_writable_file() {
    let temp_dir = TempDir::new().unwrap();
    let client = create_test_client(temp_dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "original content").unwrap();
    client.read_file(&session_id, &file_path).await.unwrap();

    // Make file read-only
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&file_path, perms).unwrap();

    let result = client
        .write_file_full(
            &session_id,
            &file_path,
            "new",
            false
        )
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        ClientError::WriteError(_) => (), // Don't assert specific inner error type
        err => panic!("Expected WriteError, got {:?}", err),
    }

    // Clean up: make file writable again so tempdir can delete it
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(&file_path, perms).unwrap();
}