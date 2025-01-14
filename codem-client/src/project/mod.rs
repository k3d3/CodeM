use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};

use crate::error::LoadError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    name: String,
    main_path: PathBuf,
    test_command: Option<String>,
}

impl Project {
    // Check if a given path is within main_path or any of the allowed_paths
    pub fn path_allowed(&self, path: &Path) -> bool {
        // First, get absolutize_from'd path
        let abs_path = path.absolutize_from(&self.main_path).unwrap();

        // Then, check if it's inside main_path
        if abs_path.starts_with(&self.main_path) {
            return true;
        }

        // Finally, check if it's a symlink, and if so, check if the target is inside main_path
        if let Ok(target) = std::fs::read_link(&abs_path) {
            if target.starts_with(&self.main_path) {
                return true;
            }
        }

        // If none of the above, return false
        false
    }

    pub fn get_base_path(&self) -> &Path {
        &self.main_path
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    projects: HashMap<String, Project>,
}

impl Projects {
    pub async fn from_file(path: &Path) -> Result<Self, LoadError> {
        let contents = std::fs::read_to_string(path)?;
        let projects: HashMap<String, Project> = toml::from_str(&contents)?;

        Ok(Self { projects })
    }

    pub fn get(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }
}
