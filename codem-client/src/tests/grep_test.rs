use std::collections::HashMap;
use std::sync::Arc;
use crate::project::Project;
use crate::session::SessionManager;
use crate::Client;
use rstest::*;
use tempfile::tempdir;
use std::fs;

#[rstest]
#[tokio::test]
async fn test_grep_file_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "test line 1\nfound this\ntest line 3").unwrap();

    let mut projects = HashMap::new();
    projects.insert("test".to_string(), Arc::new(Project::new(dir.path().to_path_buf())));
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let _session_id = client.create_session("test").await.unwrap();

    let matches = client.grep_file(&file_path, "found").await.unwrap();
    assert_eq!(matches.path, file_path);
    assert_eq!(matches.matches.len(), 1);
    assert_eq!(matches.matches[0].context, "found this");
}

#[rstest]
#[tokio::test]
async fn test_grep_file_no_match() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "test line 1\ntest line 2\ntest line 3").unwrap();

    let mut projects = HashMap::new();
    projects.insert("test".to_string(), Arc::new(Project::new(dir.path().to_path_buf())));
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let _session_id = client.create_session("test").await.unwrap();

    let matches = client.grep_file(&file_path, "nomatch").await.unwrap();
    assert_eq!(matches.path, file_path);
    assert_eq!(matches.matches.len(), 0);
}

#[rstest]
#[tokio::test]
async fn test_grep_codebase() {
    let dir = tempdir().unwrap();
    
    fs::create_dir(dir.path().join("subdir")).unwrap();
    fs::write(
        dir.path().join("file1.txt"),
        "test line 1\nfound this\ntest line 3"
    ).unwrap();
    fs::write(
        dir.path().join("subdir/file2.txt"), 
        "another line\nfound that\nlast line"
    ).unwrap();

    let mut projects = HashMap::new();
    projects.insert("test".to_string(), Arc::new(Project::new(dir.path().to_path_buf())));
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let _session_id = client.create_session("test").await.unwrap();

    let results = client.grep_codebase(dir.path(), "found").await.unwrap();
    assert_eq!(results.len(), 2);

    let mut paths: Vec<_> = results.iter()
        .map(|r| &r.path)
        .collect();
    paths.sort();
    
    assert_eq!(paths[0], &dir.path().join("file1.txt"));
    
    let first_match = results.iter()
        .find(|r| r.path == dir.path().join("file1.txt"))
        .unwrap();
    assert_eq!(first_match.matches[0].context, "found this");

    let second_match = results.iter()
        .find(|r| r.path == dir.path().join("subdir/file2.txt"))
        .unwrap();
    assert_eq!(second_match.matches[0].context, "found that");
}