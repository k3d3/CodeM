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

    let file_matches = client.grep_file(
        &session_id,
        &file_path,
        "test",
        false,
        0
    ).await.unwrap();

    // Should get one GrepFileMatch with 2 matches in its matches vector
    assert_eq!(file_matches.len(), 1);
    assert_eq!(file_matches[0].matches.len(), 2);
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
        "test\\d+",
        false,
        0
    ).await.unwrap();

    let file2_matches = client.grep_file(
        &session_id,
        &file2_path,
        "test\\d+",
        false,
        0
    ).await.unwrap();

    // Should get one GrepFileMatch for each file, each with one match
    assert_eq!(file1_matches.len(), 1);
    assert_eq!(file1_matches[0].matches.len(), 1);
    assert_eq!(file2_matches.len(), 1); 
    assert_eq!(file2_matches[0].matches.len(), 1);
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
        "non-existent",
        false,
        0
    ).await.unwrap();

    // Should get an empty vector of GrepFileMatch when no matches found
    assert_eq!(matches.len(), 0);
}
