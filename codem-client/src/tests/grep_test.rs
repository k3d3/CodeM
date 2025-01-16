use std::fs;
use tempfile::TempDir;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_grep_file() {
    // Create test directory and files
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("file1.txt");
    fs::write(&file_path, "test content\nother line\ntest pattern").unwrap();

    let client = create_test_client(dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

    let matches = client.grep_file(
        &session_id,
        &file_path,
        "test"
    ).await.unwrap();

    assert_eq!(matches.len(), 2);
}

#[tokio::test]
async fn test_grep_codebase() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("file1.txt"), "test123\nother line").unwrap();
    fs::write(dir.path().join("file2.txt"), "more content\ntest456").unwrap();
    fs::write(dir.path().join("file3.txt"), "unrelated content").unwrap();

    let client = create_test_client(dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

    let matches = client.grep_codebase(
        &session_id, 
        dir.path(),
        "test\\d+"
    ).await.unwrap();

    assert_eq!(matches.len(), 2);
}

#[tokio::test]
async fn test_grep_empty_result() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("file1.txt");
    fs::write(&file_path, "test content").unwrap();

    let client = create_test_client(dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

    let matches = client.grep_file(
        &session_id,
        &file_path,
        "non-existent"
    ).await.unwrap();

    assert_eq!(matches.len(), 0);
}