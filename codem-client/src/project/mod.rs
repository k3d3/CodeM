use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub base_path: PathBuf,
    pub allowed_paths: Option<Vec<PathBuf>>,
    pub test_command: Option<String>,
}

impl Project {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            name: "test".to_string(),
            base_path,
            allowed_paths: None,
            test_command: None,
        }
    }
}