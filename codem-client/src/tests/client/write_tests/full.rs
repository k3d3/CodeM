use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use std::sync::Arc;
use std::collections::HashMap;

use crate::{Client, project::Project, session::SessionManager};

#[rstest]
#[tokio::test]
async fn test_full_write() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "original content").unwrap();

    let mut projects = HashMap::new();
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    let result = client
        .write_file_full(
            &session_id,
            &file_path,
            "new content",
        )
        .await
        .unwrap();

    assert_eq!(result.size, "new content".len());
    assert_eq!(result.line_count, 1);
}