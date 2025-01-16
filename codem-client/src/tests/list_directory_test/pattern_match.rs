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
async fn test_list_directory_with_pattern(test_dir: TempDir) -> Result<(), anyhow::Error> {
    let mut projects = HashMap::new();
    let mut project = Project::new(test_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![test_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let session_id = client.create_session("test").await?;
    let options = ListOptions {
        file_pattern: Some(r"\.txt$".to_string()),
        recursive: true,
        ..Default::default()
    };
    
    let result = client.list_directory(&session_id, test_dir.path(), options).await?;
    
    let all_files = collect_files(&result);
    assert_eq!(all_files.len(), 2);
    assert!(all_files.contains(&"file1.txt".to_string()));
    assert!(all_files.contains(&"subdir1/file2.txt".to_string()));
    assert!(!all_files.contains(&"subdir2/file3.rs".to_string()));
    
    Ok(())
}