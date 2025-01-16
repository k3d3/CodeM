use crate::{
    Client, project::Project,
    session::SessionManager
};
use std::fs;
use std::sync::Arc;
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_path_not_allowed() {
    let temp_dir = TempDir::new().unwrap();
    let disallowed_file = temp_dir.path().parent().unwrap().join("test.txt"); 
    fs::write(&disallowed_file, "test").unwrap();

    let mut projects = HashMap::new();
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));

    let session_manager = SessionManager::new(projects, None);
    let client = Client::new(session_manager);
    let session_id = client.create_session("test").await.unwrap();

    let result = client
        .write_file(
            &session_id,
            &disallowed_file,
            "new",
        )
        .await;

    assert!(result.is_err());
}