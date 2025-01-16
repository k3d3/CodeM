use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::{config::{ClientConfig, CommandPattern}, Client, Project};

pub fn create_test_client(base_path: impl AsRef<Path>) -> Client {
    let mut projects = HashMap::new();
    let mut project = Project::new(base_path.as_ref().to_path_buf());
    project.allowed_paths = Some(vec![base_path.as_ref().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let config = ClientConfig::new(
        projects,
        vec![
            // Basic safe commands used in tests
            CommandPattern { pattern: "echo".to_string(), is_regex: false }
        ],
        vec![
            // Known risky commands
            CommandPattern { pattern: ".*rm.*".to_string(), is_regex: true }
        ]
    );

    Client::new(config)
}