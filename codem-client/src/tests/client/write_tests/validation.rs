use crate::{
    error::FileError,
    types::{CheckOptions, SessionId, WriteOperation},
    Client,
};
use std::fs;
use tempfile::TempDir;
use uuid::Uuid;

#[tokio::test]
async fn test_write_unauthorized_path() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "original content").unwrap();

    // Create client authorized for a different directory
    let other_dir = TempDir::new().unwrap();
    let client = Client::new(vec![other_dir.path().to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    let _ = client.read(&session_id, &file_path).await.unwrap();

    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Full("new content".to_string()),
            CheckOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(FileError::PathNotAllowed)));
    assert_eq!(fs::read_to_string(&file_path).unwrap(), "original content");
}

#[tokio::test]
async fn test_write_session_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "original content").unwrap();

    let client = Client::new(vec![temp_dir.path().to_path_buf()]).unwrap();
    // Use a fake session ID that doesn't exist
    let session_id = SessionId(Uuid::new_v4());

    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Full("new content".to_string()),
            CheckOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(FileError::SessionNotFound)));
    assert_eq!(fs::read_to_string(&file_path).unwrap(), "original content");
}

#[tokio::test]
async fn test_write_without_read() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "original content").unwrap();

    let client = Client::new(vec![temp_dir.path().to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Full("new content".to_string()),
            CheckOptions::default(),
        )
        .await;

    assert!(matches!(result, Err(FileError::FileNotRead)));
    assert_eq!(fs::read_to_string(&file_path).unwrap(), "original content");
}
