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
    let file1_path = dir.path().join("file1.txt");
    let file2_path = dir.path().join("file2.txt");
    fs::write(&file1_path, "test123\nother line").unwrap();
    fs::write(&file2_path, "more content\ntest456").unwrap();

    let client = create_test_client(dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

    let file1_matches = client.grep_file(
        &session_id,
        &file1_path,
        "test\\d+"
    ).await.unwrap();

    let file2_matches = client.grep_file(
        &session_id,
        &file2_path,
        "test\\d+"
    ).await.unwrap();

    // We should find one match in each file
    assert_eq!(file1_matches.len(), 1);
    assert_eq!(file2_matches.len(), 1);
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