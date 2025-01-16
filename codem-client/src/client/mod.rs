pub(crate) mod grep;
pub(crate) mod read;
pub mod write;

use std::path::Path;
use crate::{
    error::{ClientError, GrepError},
    session::SessionManager,
};
use codem_core::types::*;

pub struct Client {
    sessions: SessionManager,
}

impl Client {
    pub fn new(sessions: SessionManager) -> Self {
        Self { sessions }
    }

    pub async fn grep_file(
        &self,
        path: impl AsRef<Path>,
        pattern: &str
    ) -> Result<GrepFileMatch, GrepError> {
        grep::grep_file(path, pattern).await
    }

    pub async fn grep_codebase(
        &self,
        root_dir: impl AsRef<Path>,
        pattern: &str
    ) -> Result<Vec<GrepFileMatch>, GrepError> {
        grep::grep_codebase(root_dir, pattern).await
    }

    pub async fn create_session(&self, project_name: &str) -> Result<String, ClientError> {
        let session_id = self.sessions.create_session(project_name).await?;
        Ok(session_id.to_string())
    }
}