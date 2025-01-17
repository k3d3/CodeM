use std::path::Path;
use crate::{Client, Project, config::ClientConfig};

pub fn create_test_client(base_path: impl AsRef<Path>, test_command: Option<String>) -> Client {
    let base_dir = base_path.as_ref();
    
    // Create session directory
    let session_dir = base_dir.join("session");
    std::fs::create_dir_all(&session_dir).unwrap();
    // Create empty but valid session file
    std::fs::write(
        session_dir.join("session.toml"),
        "# Codem session file\n" // Empty TOML with comment
    ).unwrap();

    let mut project = Project::new(base_dir.to_path_buf());
    project.allowed_paths = Some(vec![base_dir.to_path_buf()]);
    project.test_command = test_command;
    let projects = vec![project];
    
    let config = ClientConfig::new(
        projects,
        base_dir.join("session").join("session.toml"),
        vec![
            // Basic safe commands used in tests
            "^echo [a-zA-Z0-9_-]+$".to_string(), // Safe pattern that doesn't allow spaces
            "exit.*".to_string()
        ],
        vec![
            // Known risky commands
            ".*rm.*".to_string()
        ]
    ).unwrap();

    Client::new(config)
}

