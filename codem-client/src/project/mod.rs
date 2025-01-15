use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub base_path: PathBuf,
    pub allowed_paths: Vec<PathBuf>,
    pub test_command: Option<String>,
}