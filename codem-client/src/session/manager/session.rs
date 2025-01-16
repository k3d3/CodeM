use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{
    project::Project,
    session::{SessionInfo, SessionId},
};
use super::{metadata::Metadata, path::PathValidator};

pub struct Session {
    pub id: String,
    pub project: Arc<Project>,
    pub metadata: Arc<RwLock<Metadata>>,
    pub path_validator: PathValidator,
}

impl Session {
    pub fn new(
        id: String,
        project: Arc<Project>,
        metadata: Metadata,
        path_validator: PathValidator,
    ) -> Self {
        Self {
            id,
            project,
            metadata: Arc::new(RwLock::new(metadata)),
            path_validator,
        }
    }

    pub fn to_info(&self) -> SessionInfo {
        SessionInfo {
            id: SessionId(self.id.clone()),
            project_name: self.project.name.clone(),
            file_timestamps: std::collections::HashMap::new(),
        }
    }
}