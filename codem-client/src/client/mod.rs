pub mod read;
pub mod write;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::fs;

use serde::{Deserialize, Serialize};

use crate::{
    error::ClientError,
    session::{SessionManager, SessionInfo},
    project::Project,
};

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    projects: Vec<Project>,
    session_persist_path: Option<PathBuf>,
}

pub struct Client {
    sessions: Arc<SessionManager>,
}

impl Client {
    pub async fn new(config_path: &Path) -> Result<Self, ClientError> {
        let contents = fs::read_to_string(config_path)
            .await
            .map_err(ClientError::from)?;
            
        let config: ClientConfig = toml::from_str(&contents).map_err(ClientError::from)?;
        
        let projects: HashMap<String, Arc<Project>> = config.projects
            .into_iter()
            .map(|p| (p.name.clone(), Arc::new(p)))
            .collect();
            
        let sessions = SessionManager::new(projects, config.session_persist_path);

        Ok(Self {
            sessions: Arc::new(sessions)
        })
    }

    pub async fn create_session(&self, project_name: &str) -> Result<String, ClientError> {
        let session_id = self.sessions.create_session(project_name).await?;
        Ok(session_id.as_str().to_string())
    }

    pub async fn get_sessions(&self) -> Vec<SessionInfo> {
        self.sessions.list_sessions().await
    }
}