use crate::{
    Client, project::Project,
    session::SessionManager
};
use std::fs;
use std::sync::Arc;
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_write_without_read() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test").unwrap();

    let mut projects = HashMap::new();
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let session_manager = SessionManager::new(projects, None);
    let client = Client::new(session_manager);
    let session_id = client.create_session("test").await.unwrap();

    let result = client
        .write_file_full(
            &session_id, 
            &file_path,
            "new",
        )
        .await;

    assert!(result.is_err());
}