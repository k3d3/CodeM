use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use crate::{
    config::ClientConfig,
    error::ClientError,
    project::Project,
    session::SessionId,
};

pub mod metadata;
pub mod session;
pub mod path;

use metadata::Metadata;
use session::Session;

pub struct SessionManager {
    config: ClientConfig,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    metadata: Arc<Mutex<Metadata>>,
    path_validator: path::PathValidator,
}

impl SessionManager {
    pub async fn new(config: ClientConfig) -> Self {
        let metadata = Metadata::new(config.session_file.clone()).await;
        let path_validator = path::PathValidator::new(config.clone());
        let metadata_arc = Arc::new(Mutex::new(metadata));

        let sessions = Self::restore_sessions(&config, metadata_arc.clone(), &path_validator).await;
        let sessions_map = Arc::new(Mutex::new(sessions));

        Self {
            config,
            sessions: sessions_map,
            metadata: metadata_arc,
            path_validator,
        }
    }

    async fn restore_sessions(
        config: &ClientConfig,
        metadata: Arc<Mutex<Metadata>>,
        path_validator: &path::PathValidator,
    ) -> HashMap<String, Session> {
        let mut sessions = HashMap::new();
        let metadata_guard = metadata.lock().await;

        for session_id in metadata_guard.get_session_ids() {
            if let Some(project_name) = metadata_guard.get_session_project(&session_id) {
                if let Some(project) = config.projects.get(&project_name).cloned() {
                    tracing::info!("Restoring session {} for project {}", session_id, project_name);
                    let session = Session::new(
                        session_id.clone(),
                        project,
                        metadata_guard.clone(),
                        path_validator.clone(),
                    );
                    sessions.insert(session_id, session);
                } else {
                    tracing::warn!("Project {} not found for session {}", project_name, session_id);
                }
            }
        }

        tracing::info!("Restored {} sessions", sessions.len());
        sessions
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Session, ClientError> {
        let sessions = self.sessions.lock().await;
        sessions.get(session_id)
            .cloned()
            .ok_or_else(|| ClientError::SessionNotFound { id: session_id.to_string() })
    }

    pub async fn create_session(&self, project: &str) -> Result<String, ClientError> {
        let project = self.get_project(project)?;
        let session_id = SessionId::new().to_string();
        
        let session = Session::new(
            session_id.clone(),
            project,
            self.metadata.lock().await.clone(),
            self.path_validator.clone(),
        );

        self.sessions.lock().await.insert(session_id.clone(), session);
        
        Ok(session_id)
    }

    pub fn get_project(&self, name: &str) -> Result<Arc<Project>, ClientError> {
        self.config.projects.get(name)
            .cloned()
            .ok_or_else(|| ClientError::ProjectNotFound { name: name.to_string() })
    }

    pub async fn check_path(&self, session_id: &str, path: &std::path::Path) -> Result<(), ClientError> {
        let session = self.get_session(session_id).await?;
        session.validate_path(path)
    }

    pub async fn get_timestamp(&self, session_id: &str, path: &std::path::Path) -> Result<std::time::SystemTime, ClientError> {
        let session = self.get_session(session_id).await?;
        session.get_timestamp(path).await
    }

    pub async fn update_timestamp(&self, session_id: &str, path: &std::path::Path, timestamp: std::time::SystemTime) -> Result<(), ClientError> {
        let session = self.get_session(session_id).await?;
        session.update_timestamp(path, timestamp).await
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
}