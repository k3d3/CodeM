mod read;
mod write;
mod grep;

use crate::error::Result;
use crate::types::{Session, SessionId};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

pub struct Client {
    allowed_dirs: Vec<PathBuf>,
    sessions: RwLock<Vec<Session>>,
}

impl Client {
    /// Create a new Client instance
    pub fn new(allowed_dirs: Vec<PathBuf>) -> Result<Self> {
        Ok(Self {
            allowed_dirs,
            sessions: RwLock::new(Vec::new()),
        })
    }

    /// Create a new session
    pub async fn create_session(&self, project_name: &str) -> Result<SessionId> {
        let session = Session::new(project_name.to_string(), self.allowed_dirs.clone());
        let session_id = session.id().clone();
        
        let mut sessions = self.sessions.write().await;
        sessions.push(session);
        
        Ok(session_id)
    }

    /// List allowed paths
    pub fn list_allowed_paths(&self) -> Vec<PathBuf> {
        self.allowed_dirs.clone()
    }

    /// Check if a path is allowed by verifying it is under one of the allowed directories
    pub(crate) fn is_path_allowed(&self, path: &Path) -> bool {
        let normalized = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // If the path doesn't exist, check its parent directory
                match path.parent().and_then(|p| p.canonicalize().ok()) {
                    Some(parent) => parent,
                    None => return false,
                }
            }
        };

        self.allowed_dirs.iter().any(|allowed| {
            match allowed.canonicalize() {
                Ok(allowed_canon) => normalized.starts_with(allowed_canon),
                Err(_) => false,
            }
        })
    }

    #[cfg(test)]
    /// Get all active sessions (for testing)
    pub(crate) async fn get_sessions(&self) -> Vec<Session> {
        self.sessions.read().await.clone()
    }
}