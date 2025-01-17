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
    pub fn into_client_config(self) -> Result<ClientConfig, ConfigError> {
        ClientConfig::new(
            self.projects,
            self.session_file,
            self.safe_patterns,
            self.risky_patterns
        )
    }
}