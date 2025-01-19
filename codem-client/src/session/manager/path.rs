use std::path::{Path, PathBuf};
use crate::{error::ClientError, config::ClientConfig};

#[derive(Clone)]
pub struct PathValidator {
    config: ClientConfig,
}

impl PathValidator {
    pub fn to_relative_path(&self, path: &Path) -> PathBuf {
        for project in self.config.projects.values() {
            if let Ok(relative) = path.strip_prefix(&project.base_path) {
                return relative.to_path_buf();
            }
            
            if let Some(ref allowed_paths) = project.allowed_paths {
                for allowed_path in allowed_paths {
                    if let Ok(relative) = path.strip_prefix(allowed_path) {
                        return relative.to_path_buf();
                    }
                }
            }
        }
        path.to_path_buf()
    }

    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub fn validate_path(&self, path: &Path) -> Result<(), ClientError> {
        // Convert to absolute path if relative
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // Check if path is within any allowed directory
        for project in self.config.projects.values() {
            let base_path = &project.base_path;
            if path.starts_with(base_path) {
                return Ok(());
            }

            // Check additional allowed paths
            if let Some(ref allowed_paths) = project.allowed_paths {
                for allowed_path in allowed_paths {
                    if path.starts_with(allowed_path) {
                        return Ok(());
                    }
                }
            }
        }

        Err(ClientError::PathOutOfScope { 
            path,
        })
    }
}
