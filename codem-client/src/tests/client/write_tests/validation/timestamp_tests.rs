use tempfile::TempDir;
use std::fs;
use tokio::time::{sleep, Duration};
use crate::error::ClientError;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_timestamp_updated_on_failed_write() {
    let temp_dir = TempDir::new().unwrap();
    let client = create_test_client(temp_dir.path(), None).await;
    
    // Create session and initialize file
    let session_id = client.create_session("test").await.unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "original content").unwrap();
    
    // Read file to get initial timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    // Modify file outside of client to cause timestamp mismatch
    sleep(Duration::from_millis(10)).await; // Ensure timestamp changes
    fs::write(&file_path, "modified content").unwrap();

    // Try to write - this should fail but still record current timestamp
    let first_write = client
        .write_file_full(&session_id, &file_path, "new content", false)
        .await;

    // Check error was returned with correct content
    assert!(first_write.is_err());
    match first_write.unwrap_err() {
        ClientError::FileModifiedSinceRead { content } => {
            assert_eq!(content.unwrap(), "modified content");
        },
        err => panic!("Expected FileModifiedSinceRead, got {:?}", err),
    }

    // Compare stored timestamp vs filesystem
    let new_stored_ts = client
        .sessions.get_session(&session_id).await.unwrap()
        .get_timestamp(&file_path).await;
        
    let fs_ts = fs::metadata(&file_path).unwrap().modified().unwrap();
    
    println!("new_stored_ts: {:?}", new_stored_ts);
    println!("fs_ts: {:?}", fs_ts);

    // Second write should succeed since timestamp was updated
    let second_write = client
        .write_file_full(&session_id, &file_path, "newer content", false)
        .await;
    
    assert!(second_write.is_ok());
}