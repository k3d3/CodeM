use std::{path::Path, sync::Arc, time::SystemTime};
use tokio::sync::Mutex;
use crate::{
    error::ClientError,
    project::Project, 
    session::{SessionInfo, SessionId},
};
use super::{metadata::Metadata, path::PathValidator};
use tokio::fs;

#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub project: Arc<Project>,
    pub metadata: Arc<Mutex<Metadata>>,
    pub path_validator: PathValidator,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("id", &self.id)
            .field("project", &self.project)
            .finish()
    }
}

impl Session {
    pub fn new(
        id: String,
        project: Arc<Project>,
        metadata: Metadata,
        path_validator: PathValidator,
    ) -> Self {
        // We need to ensure the project is stored in metadata when creating a new session
        tokio::spawn({
            let metadata = Arc::new(Mutex::new(metadata.clone()));
            let id = id.clone();
            let project_name = project.name.clone();
            
            async move {
                if let Some(existing_project) = metadata.lock().await.get_session_project(&id) {
                    if existing_project != project_name {
                        // Project name mismatch - this shouldn't happen in normal operation
                        tracing::error!(
                            "Session {} has mismatched project names: {} vs {}",
                            id, existing_project, project_name
                        );
                    }
                }
            }
        });

        Self {
            id,
            project,
            metadata: Arc::new(Mutex::new(metadata)),
            path_validator,
        }
    }

    pub fn validate_path(&self, path: &Path) -> Result<(), ClientError> {
        self.path_validator.validate_path(path)
    }

    pub async fn get_timestamp(&self, path: &Path) -> Result<SystemTime, ClientError> {
        // Read file content first so we have it for potential error case
        let file_content = fs::read_to_string(path).await.ok();
        
        let metadata = self.metadata.lock().await;
        metadata.get_timestamp(&self.id, path).map_err(|e| {
            if let ClientError::FileNotSynced { .. } = e {
                ClientError::FileNotSynced { 
                    content: file_content
                }
            } else {
                e
            }
        })
    }

    pub async fn update_timestamp(&self, path: &Path, timestamp: SystemTime) -> Result<(), ClientError> {
        let mut metadata = self.metadata.lock().await;
        metadata.update_session(&self.id, &self.project.name, path, timestamp).await
    }

    pub async fn to_info(&self) -> SessionInfo {
        SessionInfo {
            id: SessionId(self.id.clone()),
            project_name: self.project.name.clone(),
            file_timestamps: {
                let metadata = self.metadata.lock().await;
                metadata.get_session_timestamps(&self.id)
            },
        }
    }
}