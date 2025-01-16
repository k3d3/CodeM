use std::fs;
use tempfile::TempDir;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_write_without_read() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt"); 
    fs::write(&file_path, "test").unwrap();

    let client = create_test_client(temp_dir.path());
    let session_id = client.create_session("test").await.unwrap();

    // Write without reading first
    let result = client
        .write_file_full(
            &session_id,
            &file_path,
            "new",
        )
        .await;

    // Expect error because we haven't cached the timestamp 
    assert!(result.is_err());
}