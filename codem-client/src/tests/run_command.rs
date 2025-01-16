use std::collections::HashMap;
use std::sync::Arc;
use std::path::PathBuf;
use rstest::rstest;

use crate::{Client, session::SessionManager, Project};

#[rstest]
#[tokio::test] 
async fn test_run_command() {
    let mut projects = HashMap::new();
    let project = Project::new(PathBuf::from("/tmp"));
    projects.insert(project.name.clone(), Arc::new(project));
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let session_id = client.create_session("test").await.unwrap();
    
    // Test successful command
    let result = client.run_command(&session_id, "echo 'hello'", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.stdout.trim(), "hello");

    // Test failed command
    let result = client.run_command(&session_id, "nonexistent-command", None).await;
    assert!(result.is_err());
}

#[rstest]
#[tokio::test]
async fn test_run_command_risky() {
    let mut projects = HashMap::new();
    let project = Project::new(PathBuf::from("/tmp"));
    projects.insert(project.name.clone(), Arc::new(project));
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    
    let session_id = client.create_session("test").await.unwrap();
    
    // Test successful command
    let result = client.run_command_risky(&session_id, "echo 'hello'", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.stdout.trim(), "hello");
    assert_eq!(output.exit_code, 0);

    // Test failed command  
    let result = client.run_command_risky(&session_id, "nonexistent-command", None).await;
    assert!(result.is_err());
}