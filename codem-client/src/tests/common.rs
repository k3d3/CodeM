use std::fs;
use std::path::Path;

use crate::{Client, Project, config::ClientConfig};

pub fn create_test_client(base_path: impl AsRef<Path>) -> Client {
    let tmp_dir = std::env::temp_dir().join("codem_test");
    fs::create_dir_all(&tmp_dir).unwrap();

    let mut project = Project::new(base_path.as_ref().to_path_buf());
    project.allowed_paths = Some(vec![base_path.as_ref().to_path_buf()]);
    let projects = vec![project];
    
    let config = ClientConfig::new(
        projects,
        tmp_dir.join("session.toml"),
        vec![
            // Basic safe commands used in tests
            "echo".to_string()
        ],
        vec![
            // Known risky commands
            ".*rm.*".to_string()
        ]
    ).unwrap();

    Client::new(config)
}