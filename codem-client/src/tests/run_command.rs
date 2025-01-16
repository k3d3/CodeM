use std::fs;
use rstest::rstest;
use crate::{Client, Project, config::ClientConfig};

#[rstest]
#[tokio::test] 
async fn test_run_command() {
    let temp_dir = std::env::temp_dir().join("codem_test");
    fs::create_dir_all(&temp_dir).unwrap();
    fs::create_dir_all(&temp_dir.join("session")).unwrap();

    let mut project = Project::new(temp_dir.clone());
    project.allowed_paths = Some(vec![temp_dir.clone()]);
    let projects = vec![project];

    let config = ClientConfig::new(
        projects,
        temp_dir.join("session").join("session.toml"),
        vec!["echo [a-zA-Z0-9-_ ]".to_string()], 
        vec![]
    ).unwrap();

    let client = Client::new(config);
    
    let session_id = client.create_session("test").await.unwrap();
    
    let result = client.run_command(&session_id, "echo hello", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.stdout.trim(), "hello");

    drop(client); // Ensure client is dropped before removing directory
    let _ = fs::remove_dir_all(temp_dir); // Ignore remove errors
}