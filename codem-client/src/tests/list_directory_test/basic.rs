use crate::Client;
use super::*;
use crate::{
    session::SessionManager,
    project::Project
};
use std::collections::HashMap;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_list_directory_basic(test_dir: TempDir) -> Result<(), anyhow::Error> {
    let mut projects = HashMap::new();
    let mut project = Project::new(test_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![test_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let session_id = client.create_session("test").await?;
    let options = ListOptions::default();
    
    let result = client.list_directory(&session_id, test_dir.path(), options).await?;
    
    assert!(result.entry.is_dir);
    assert_eq!(result.children.len(), 3); // subdir1, subdir2, file1.txt
    
    let file_names: Vec<String> = result.children.iter()
        .map(|entry| entry.entry.path.to_str().unwrap().to_string())
        .collect();
        
    assert!(file_names.contains(&"file1.txt".to_string()));
    assert!(file_names.contains(&"subdir1".to_string()));
    assert!(file_names.contains(&"subdir2".to_string()));
    
    Ok(())
}