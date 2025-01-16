use crate::{Client, config::ClientConfig}; 
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_read_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let content = "test content";
    fs::write(&file_path, content).unwrap();

    let temp_path = temp_dir.path();
    let client = create_test_client(temp_path);
    let session_id = client.create_session("test").await.unwrap();

    let result = client.read_file(&session_id, &file_path).await.unwrap();

    assert_eq!(result.content, content);
}

#[tokio::test]
async fn test_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let client = create_test_client(temp_path);
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

    let client = create_test_client(temp_path);
    let session_id = client.create_session("test").await.unwrap();

    let result = client.read_file(&session_id, &file_path).await.unwrap();

    assert_eq!(result.content, large_content);
}

fn create_test_client(base_path: impl AsRef<std::path::Path>) -> Client {
    // Setup client config
    let mut project = crate::project::Project::new(base_path.as_ref().to_path_buf());
    project.allowed_paths = Some(vec![base_path.as_ref().to_path_buf()]);
    let projects = vec![project];
    
    let tmp_dir = std::env::temp_dir().join("codem_test");
    fs::create_dir_all(&tmp_dir).unwrap();
    
    let config = ClientConfig::new(
        projects,
        tmp_dir.join("session.toml"),
        vec!["*.txt".to_string()],
        vec![]
    ).unwrap();

    Client::new(config)
}