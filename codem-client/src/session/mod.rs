use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use parking_lot::RwLock;
use uuid::Uuid;

use crate::{
    error::{ClientError, OperationError, SessionError}, 
    project::Project,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub project_name: String,
    pub id: SessionId,
    pub file_timestamps: HashMap<PathBuf, std::time::SystemTime>,
}

#[derive(Debug)]
pub struct SessionManager {
    projects: HashMap<String, Arc<Project>>,
    sessions: RwLock<HashMap<SessionId, Arc<SessionInfo>>>,
}

impl SessionManager {
    pub fn new(
        projects: HashMap<String, Arc<Project>>,  
        _persist_path: Option<PathBuf>,
    ) -> Self {
        Self {
            projects,
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create_session(&self, project_name: &str) -> Result<SessionId, ClientError> {
        let project = self.projects.get(project_name).ok_or_else(|| {
            SessionError::SessionNotFound { 
                id: project_name.to_string() 
            }
        })?;

        let session_id = SessionId::new();
        
        let info = SessionInfo {
            project_name: project_name.to_string(),
            id: session_id.clone(),
            file_timestamps: HashMap::new(),
        };

        self.sessions.write().insert(session_id.clone(), Arc::new(info));
        
        Ok(session_id)
    }

    pub async fn list_sessions(&self) -> Vec<SessionInfo> {
        self.sessions.read()
            .values()
            .map(|s| (**s).clone())
            .collect()
    }

    pub fn get_session(&self, session_id: &str) -> Result<Arc<SessionInfo>, ClientError> {
        let id = SessionId(session_id.to_string());
        self.sessions.read()
            .get(&id)
            .cloned()
            .ok_or_else(|| SessionError::SessionNotFound {
                id: session_id.to_string()
            }.into())
    }

    pub fn get_project(&self, session_id: &str) -> Result<Arc<Project>, ClientError> {
        let session = self.get_session(session_id)?;
        self.projects.get(&session.project_name)
            .cloned()
            .ok_or_else(|| SessionError::SessionNotFound {
                id: session.project_name.clone()
            }.into())
    }

    pub fn check_path(&self, session_id: &str, path: &Path) -> Result<(), ClientError> {
        let project = self.get_project(session_id)?;
        let mut allowed = false;
        
        if let Some(parent) = path.parent() {
            if parent.starts_with(&project.base_path) {
                allowed = true;
            }
        }

        if !allowed {
            for allowed_path in &project.allowed_paths {
                if path.starts_with(allowed_path) {
                    allowed = true;
                    break;
                }
            }
        }

        if !allowed {
            return Err(OperationError::PathNotAllowed {
                path: path.to_string_lossy().into_owned()
            }.into());
        }

        Ok(())
    }

    pub fn update_timestamp(
        &self, 
        session_id: &str,
        path: &Path,
        timestamp: std::time::SystemTime
    ) -> Result<(), ClientError> {
        let session = self.get_session(session_id)?;
        let mut sessions = self.sessions.write();
        
        if let Some(info) = sessions.get_mut(&session.id) {
            let mut info = (**info).clone();
            info.file_timestamps.insert(path.to_path_buf(), timestamp);
            sessions.insert(session.id.clone(), Arc::new(info));
        }
        
        Ok(())
    }

    pub fn check_timestamp(&self, session_id: &str, path: &Path) -> Result<(), ClientError> {
        let session = self.get_session(session_id)?;
        
        if let Some(stored) = session.file_timestamps.get(path) {
            let metadata = path.metadata().map_err(|e| OperationError::IoError(e))?;
            let current = metadata.modified().map_err(|e| OperationError::IoError(e))?;
            
            if current != *stored {
                return Err(OperationError::TimestampMismatch {
                    path: path.to_string_lossy().into_owned()
                }.into());
            }
        }
        
        Ok(())
    }
}