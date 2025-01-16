use std::{collections::HashMap, sync::Arc, path::PathBuf};
use crate::{
    error::ClientError,
    project::Project,
    session::{SessionId, SessionInfo, SessionManager},
};
use parking_lot::RwLock;

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
        let _project = self.projects.get(project_name).ok_or_else(|| ClientError::ProjectNotFound {
            name: project_name.to_string()
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
        self.sessions.read().values().map(|s| (**s).clone()).collect()
    }

    pub fn get_session(&self, session_id: &str) -> Result<Arc<SessionInfo>, ClientError> {
        let id = SessionId(session_id.to_string());
        self.sessions.read().get(&id).cloned().ok_or_else(|| ClientError::SessionNotFound {
                id: session_id.to_string().into_boxed_str(),
        })
    }

    pub fn get_project(&self, session_id: &str) -> Result<Arc<Project>, ClientError> {
        let session = self.get_session(session_id)?;
        self.projects.get(&session.project_name).cloned().ok_or_else(|| ClientError::ProjectNotFound {
                name: session.project_name.to_string()
        })
    }
}