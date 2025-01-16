pub mod command;
pub mod file;
pub mod read;
pub mod write;

use crate::{error::ClientError, config::ClientConfig};
use crate::session::manager::SessionManager;
use crate::project::Project;
use codem_core::types::{GrepMatch, GrepOptions};

use std::path::Path;
use std::sync::Arc;

pub struct Client {
    pub(crate) sessions: SessionManager,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            sessions: SessionManager::new(config)
        }
    }

    pub async fn create_session(&self, project: &str) -> Result<String, ClientError> {
        self.sessions.create_session(project).await
    }

    pub async fn grep_file(
        &self,
        session_id: &str,
        path: impl AsRef<Path>,
        pattern: &str,
    ) -> Result<Vec<GrepMatch>, ClientError> {
        self.sessions.check_path(session_id, path.as_ref())?;
        let pattern = regex::Regex::new(pattern).map_err(|_| ClientError::InvalidPath {
            path: path.as_ref().to_path_buf(),
        })?;
        
        let opts = GrepOptions::default();
        let result = codem_core::grep::grep_file(path.as_ref(), &pattern, &opts).await?;
        Ok(result.map(|m| m.matches).unwrap_or_default())
    }

    pub async fn grep_codebase(
        &self,
        session_id: &str,
        path: impl AsRef<Path>,
        pattern: &str,
    ) -> Result<Vec<GrepMatch>, ClientError> {
        self.sessions.check_path(session_id, path.as_ref())?;
        let pattern = regex::Regex::new(pattern).map_err(|_| ClientError::InvalidPath {
            path: path.as_ref().to_path_buf(),
        })?;
        let opts = GrepOptions::default();
        let result = codem_core::grep::grep_file(path.as_ref(), &pattern, &opts).await?;
        Ok(result.map(|m| m.matches).unwrap_or_default())
    }

    pub(crate) fn get_project(&self, name: &str) -> Result<Arc<Project>, ClientError> {
        self.sessions.get_project(name)
    }
}