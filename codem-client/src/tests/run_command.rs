use std::fs;
use rstest::rstest;

use crate::{Client, Project, config::ClientConfig};

#[rstest]
#[tokio::test] 
async fn test_run_command() {
    let tmp_dir = std::env::temp_dir().join("codem_test");
    fs::create_dir_all(&tmp_dir).unwrap();

    let mut project = Project::new(tmp_dir.clone());
    project.allowed_paths = Some(vec![tmp_dir.clone()]);
    let projects = vec![project];

    let config = ClientConfig::new(
        projects,
        tmp_dir.join("session.toml"),
        vec!["echo".to_string()], 
        vec![]
    ).unwrap();

    let client = Client::new(config);
    
    let session_id = client.create_session("test").await.unwrap();
    
    // Test basic echo command
    let result = client.run_command(&session_id, "echo 'hello'", Some(&tmp_dir)).await;
    if let Err(err) = &result {
        println!("Command error: {:?}", err);
    }
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.stdout.trim(), "hello");

    fs::remove_dir_all(tmp_dir).unwrap();
}