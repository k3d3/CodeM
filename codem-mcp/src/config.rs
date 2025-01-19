use std::path::PathBuf;
use serde::Deserialize;
use codem_client::{config::ClientConfig, Project, error::ConfigError};

/// TOML format that can be converted into a ClientConfig
#[derive(Deserialize)]
pub struct TomlConfig {
    pub projects: Vec<Project>,
    pub session_file: PathBuf,
    pub safe_patterns: Vec<String>,
    pub risky_patterns: Vec<String>,
}

impl TomlConfig {
    pub async fn into_client_config(self) -> Result<ClientConfig, ConfigError> {
        // Create session file parent directory if it doesn't exist
        if let Some(parent) = self.session_file.parent() {
            // Handle directory creation - ignore error if directory exists
            if let Err(e) = tokio::fs::create_dir_all(parent).await {
                if e.kind() != std::io::ErrorKind::AlreadyExists {
                    return Err(ConfigError::InvalidSessionFile { 
                        path: self.session_file.clone() 
                    });
                }
            }
        }

        ClientConfig::new(
            self.projects,
            self.session_file,
            self.safe_patterns,
            self.risky_patterns
        )
    }
}