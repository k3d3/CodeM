use std::collections::HashMap;
use std::sync::Arc;
use std::path::PathBuf;
use rstest::rstest;

use crate::{Client, Project, config::{ClientConfig, CommandPattern}};

#[rstest]
#[tokio::test] 
async fn test_run_command() {
    let mut projects = HashMap::new();
    let project = Project::new(PathBuf::from("/tmp"));
    projects.insert(project.name.clone(), Arc::new(project));

    let config = ClientConfig::new(
        projects,
        vec![CommandPattern { 
            pattern: "echo".to_string(), 
            is_regex: false 
        }],
        vec![]
    );

    let client = Client::new(config);
    
    let session_id = client.create_session("test").await.unwrap();
    
    // Test successful safe command
    let result = client.run_command(&session_id, "echo 'hello'", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.stdout.trim(), "hello");

    // Test unsafe command is rejected
    let result = client.run_command(&session_id, "ls -l", None).await;
    assert!(result.is_err());
}

#[rstest]
#[tokio::test]
async fn test_run_command_risky() {
    let mut projects = HashMap::new();
    let project = Project::new(PathBuf::from("/tmp"));
    projects.insert(project.name.clone(), Arc::new(project));

    let config = ClientConfig::new(
        projects,
        vec![],
        vec![] // No risky commands defined
    );

    let client = Client::new(config);
    
    let session_id = client.create_session("test").await.unwrap();
    
    // Unsafe command works with run_command_risky
    let result = client.run_command_risky(&session_id, "echo 'hello'", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.stdout.trim(), "hello");
    assert_eq!(output.exit_code, 0);

    // Invalid command fails with run_command_risky
    let result = client.run_command_risky(&session_id, "nonexistent-command", None).await;
    assert!(result.is_err());
}

#[rstest]
#[tokio::test]
async fn test_risky_command_patterns() {
    let mut projects = HashMap::new();
    let project = Project::new(PathBuf::from("/tmp")); 
    projects.insert(project.name.clone(), Arc::new(project));

    let config = ClientConfig::new(
        projects,
        vec![
            CommandPattern {
                pattern: "echo".to_string(),
                is_regex: false
            }
        ],
        vec![
            CommandPattern {
                pattern: ".*rm.*".to_string(),
                is_regex: true
            }
        ]
    );

    let client = Client::new(config);
    
    let session_id = client.create_session("test").await.unwrap();
    
    // Command with risky pattern only works with run_command_risky
    let result = client.run_command(&session_id, "rm file", None).await;
    assert!(result.is_err());
    
    let result = client.run_command_risky(&session_id, "rm file", None).await;
    assert!(result.is_err()); // Command fails but is allowed to run
}