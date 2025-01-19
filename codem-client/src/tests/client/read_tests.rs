use crate::tests::common::create_test_client;
use tempfile::TempDir;
use std::fs;
use rstest::rstest;

#[rstest]
#[tokio::test]
async fn test_read_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let content = "test content";
    fs::write(&file_path, content).unwrap();

    let temp_path = temp_dir.path();
    let client = create_test_client(temp_path, None).await;
    let session_id = client.create_session("test").await.unwrap();

    // Reading should succeed and match contents
    let (result, _metadata) = client.read_file(&session_id, &file_path).await.unwrap();
    assert_eq!(result, content);

    // Modify file externally
    let new_content = "modified content";
    fs::write(&file_path, new_content).unwrap();
    
    // Read should succeed with new content since reads don't check timestamps
    let (result2, _metadata) = client.read_file(&session_id, &file_path).await.unwrap();
    assert_eq!(result2, new_content);
}

#[tokio::test]
async fn test_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let client = create_test_client(temp_path, None).await;
    let session_id = client.create_session("test").await.unwrap();

    let result = client.read_file(
        &session_id,
        &temp_path.join("nonexistent.txt")
    ).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");
    let temp_path = temp_dir.path();

    // Create large file
    let large_content = "x".repeat(1_000_000);
    fs::write(&file_path, &large_content).unwrap();

    let client = create_test_client(temp_path, None).await;
    let session_id = client.create_session("test").await.unwrap();

    let (result, _metadata) = client.read_file(&session_id, &file_path).await.unwrap();

    assert_eq!(result, large_content);
}