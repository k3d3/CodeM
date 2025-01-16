use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use crate::{
    config::ClientConfig,
    error::ClientError,
    project::Project,
    session::{SessionId},
};

pub mod metadata;
pub mod session;
pub mod path;

use metadata::Metadata;
use session::Session;

pub struct SessionManager {
    config: ClientConfig,
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    metadata: Arc<RwLock<Metadata>>,
    path_validator: path::PathValidator,
}

impl SessionManager {
    pub fn new(config: ClientConfig) -> Self {
        let metadata = Metadata::new(config.session_file.clone());
        Self {
            config: config.clone(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(metadata)),
            path_validator: path::PathValidator::new(config),
        }
    }

    pub async fn create_session(&self, project: &str) -> Result<String, ClientError> {
        let project = self.get_project(project)?;
        let session_id = SessionId::new().to_string();
        
        let session = Session::new(
            session_id.clone(),
            project,
            self.metadata.write().await.clone(),
            self.path_validator.clone(),
        );

        self.sessions.write().await.insert(session_id.clone(), session);
        
        Ok(session_id)
    }

    pub fn get_project(&self, name: &str) -> Result<Arc<Project>, ClientError> {
        self.config.projects.get(name)
            .cloned()
            .ok_or_else(|| ClientError::ProjectNotFound { name: name.to_string() })
    }

    pub fn check_path(&self, _session_id: &str, path: &std::path::Path) -> Result<(), ClientError> {
        Ok(self.path_validator.validate_path(path)?)
    }

    pub async fn get_timestamp(&self, session_id: &str, path: &std::path::Path) -> Result<std::time::SystemTime, ClientError> {
        let metadata = self.metadata.read().await;
        Ok(metadata.get_timestamp(session_id, path)?)
    }

    pub async fn update_timestamp(&self, session_id: &str, path: &std::path::Path, timestamp: std::time::SystemTime) -> Result<(), ClientError> {
        let mut metadata = self.metadata.write().await;
        Ok(metadata.update_timestamp(session_id, path, timestamp)?)
    }
}