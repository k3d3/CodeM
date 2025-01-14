mod grep;
mod read;
mod write;

use crate::{
    error::{ClientError, LoadError},
    project::Projects,
    session::{Session, SessionId, Sessions},
};
use codem_core::types::{ListEntry, ListOptions};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::sync::MutexGuard;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    projects: Projects,
    session_persist_path: PathBuf,
}

pub struct Client {
    sessions: Sessions,
}

impl Client {
    /// Create a new Client instance
    pub async fn new(config_path: &Path) -> Result<Self, ClientError> {
        let config = Self::load_config(config_path).await?;
        let projects = config.projects;
        let persist_path = config.session_persist_path;

        Ok(Self {
            sessions: Sessions::new(projects, persist_path).await?,
        })
    }

    /// Create a new session
    pub async fn create_session(&mut self, project_name: &str) -> Result<SessionId, ClientError> {
        let session = self.sessions.create_session(project_name).await?;
        Ok(session)
    }

    /// List contents of a directory with optional filtering and stats
    pub async fn list_directory(
        &self,
        session_id: &SessionId,
        path: &Path,
        options: &ListOptions,
    ) -> Result<Vec<ListEntry>, ClientError> {
        let session = self.get_session(session_id).await?;

        // Check if path is allowed
        if !session.path_allowed(path.as_ref()) {
            return Err(ClientError::PathNotAllowed);
        }

        let base_path = session.get_base_path();

        let entries = codem_core::list_directory(base_path, path, options).await?;

        Ok(entries)
    }

    async fn get_session(
        &self,
        session_id: &SessionId,
    ) -> Result<MutexGuard<Session>, ClientError> {
        self.sessions
            .get(session_id)
            .await
            .ok_or(ClientError::SessionNotFound)
    }

    async fn load_config(config_path: &Path) -> Result<ClientConfig, LoadError> {
        let contents = std::fs::read_to_string(config_path)?;
        let config: ClientConfig = toml::from_str(&contents)?;

        Ok(config)
    }
}
