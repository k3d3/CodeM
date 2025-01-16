mod write_tests;

use std::collections::HashMap;
use std::sync::Arc;
use crate::client::Client;
use crate::project::Project;
use crate::session::SessionManager;
use rstest::*;
use tempfile::tempdir;
use std::fs;

#[rstest]
#[tokio::test]
async fn test_grep_integration() {
    let dir = tempdir().unwrap();
    
    fs::write(
        dir.path().join("test.txt"),
        "line one\nfind me\nline three"
    ).unwrap();

    let mut projects = HashMap::new();
    let mut project = Project::new(dir.path().to_path_buf());
    project.allowed_paths = Some(vec![dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    let _session_id = client.create_session("test").await.unwrap();

    let result = client.grep_file(
        dir.path().join("test.txt"), 
        "find"
    ).await.unwrap();
    
    assert_eq!(result.matches.len(), 1);
    assert_eq!(result.matches[0].context, "find me");
    assert_eq!(result.matches[0].line_number, 2);

    let results = client.grep_codebase(dir.path(), "find").await.unwrap();
    assert_eq!(results.len(), 1);
    let first = &results[0];
    assert_eq!(first.matches[0].context, "find me");
    assert_eq!(first.matches[0].line_number, 2);
}