use crate::{
    error::FileError,
    types::{CheckOptions, WriteOperation},
    Client,
};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_partial_write_pattern_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "some content").unwrap();

    let client = Client::new(vec![temp_dir.path().to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    let initial_read = client.read(&session_id, &file_path).await;
    assert!(initial_read.is_ok());

    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Partial {
                old_str: "nonexistent".to_string(),
                new_str: "new".to_string(),
            },
            CheckOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(FileError::PatternNotFound)));
    assert_eq!(fs::read_to_string(&file_path).unwrap(), "some content");
}

#[tokio::test]
async fn test_partial_write_multiple_matches() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let test_content = "test test".to_string();
    fs::write(&file_path, &test_content).unwrap();
    println!("Initial content: {:?}", test_content);

    let client = Client::new(vec![temp_dir.path().to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    let initial_read = client.read(&session_id, &file_path).await;
    println!("Initial read result: {:?}", initial_read);
    assert!(initial_read.is_ok());

    let sessions = client.get_sessions().await;
    println!("Current sessions: {:?}", sessions);

    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Partial {
                old_str: "test".to_string(),
                new_str: "new".to_string(),
            },
            CheckOptions::default(),
        )
        .await;

    println!("Result: {:?}", result);
    assert!(matches!(result, Err(FileError::MultipleMatches)));
    assert_eq!(fs::read_to_string(&file_path).unwrap(), test_content);
}
