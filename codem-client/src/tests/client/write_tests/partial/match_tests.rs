use crate::{
    Client, project::Project,
    session::SessionManager
};
use codem_core::types::Change;
use std::fs;
use std::sync::Arc;
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_multiple_matches() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test content test test").unwrap();

    let mut projects = HashMap::new();
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let session_manager = SessionManager::new(projects, None);
    let client = Client::new(session_manager);
    let session_id = client.create_session("test").await.unwrap();

    let _ = client.read_file(&session_id, &file_path).await.unwrap();

    let changes = vec![Change {
        old_str: "test".to_string(),
        new_str: "new".to_string(),
        allow_multiple_matches: false,
    }];

    let result = client
        .write_file_partial(
            &session_id,
            &file_path,
            changes,
        )
        .await;

    assert!(result.is_err());
}