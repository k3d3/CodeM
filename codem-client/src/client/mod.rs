pub mod command;
pub mod read;
pub mod write;

use crate::{error::ClientError, config::ClientConfig};
use crate::session::manager::SessionManager;
use codem_core::types::{GrepFileMatch, GrepOptions};
use std::path::Path;

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
        case_sensitive: bool,
        context_lines: usize,
    ) -> Result<Vec<GrepFileMatch>, ClientError> {
        let path = path.as_ref();
        
        // Get session to access project
        let session = self.sessions.get_session(session_id).await?;
        
        // Resolve path relative to project base path
        let absolute_path = session.project.base_path.join(path);
        
        // Validate the path
        self.sessions.check_path(session_id, &absolute_path).await?;
        
        let pattern = regex::Regex::new(pattern).map_err(|_| ClientError::InvalidPath {
            path: absolute_path.clone(),
        })?;
        
        let opts = GrepOptions {
            case_sensitive,
            context_lines,
            ..Default::default()
        };

        let result = codem_core::grep::grep_file(&absolute_path, &pattern, &opts).await?;
        Ok(result.into_iter().collect())
    }

    pub async fn grep_codebase(
        &self,
        session_id: &str,
        path: Option<&Path>,
        file_pattern: Option<&str>,
        pattern: &str,
        case_sensitive: bool,
        context_lines: usize,
    ) -> Result<Vec<GrepFileMatch>, ClientError> {
        // Get session to access project
        let session = self.sessions.get_session(session_id).await?;
        
        // Resolve path relative to project base path
        let absolute_path = if let Some(path) = path {
            session.project.base_path.join(path)
        } else {
            session.project.base_path.clone()
        };
        
        // Validate the path
        self.sessions.check_path(session_id, &absolute_path).await?;

        let pattern = regex::Regex::new(pattern).map_err(|_| ClientError::InvalidPath {
            path: absolute_path.clone(),
        })?;
        
        let opts = GrepOptions {
            file_pattern: file_pattern.map(ToString::to_string),
            case_sensitive,
            context_lines
        };
        
        let matches = codem_core::grep::grep_codebase(&absolute_path, &pattern, &opts).await?;
        Ok(matches)
    }
}
