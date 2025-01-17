use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use crate::tests::common::create_test_client;
use crate::error::ClientError;

#[rstest]
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
    let file_path = temp_dir.path().join("new_file.txt");

    let result = client
        .write_file_full(
            &session_id,
            &file_path,
            "new content",
            false
        )
        .await;

    match result {
        Ok(_) => panic!("Expected write to nonexistent file to fail"),
        Err(ClientError::FileNotFound { .. }) => (), // Expected error
        err => panic!("Expected FileNotFound, got {:?}", err)
    }

    // Try to write to nonexistent directory
    let bad_path = temp_dir.path().join("nonexistent_dir").join("file.txt");
    let result = client
        .write_file_full(
            &session_id,
            &bad_path, 
            "new content",
            false
        )
        .await;

    match result {
        Ok(_) => panic!("Expected write to nonexistent directory to fail"),
        Err(ClientError::FileNotFound { .. }) => (), // Expected error  
        err => panic!("Expected FileNotFound, got {:?}", err)
    }
}