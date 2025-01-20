use std::path::Path;
use codem_core::{types::TreeEntry, error::DirectoryError};
use crate::{Client, error::ClientError};

impl Client {
    pub async fn list_directory(
        &self,
        session_id: &str,
        path: Option<&Path>,
        options: codem_core::types::ListOptions,
    ) -> Result<TreeEntry, ClientError> {
        let path = path.as_ref();

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

        // List directory using codem_core
        let tree = codem_core::directory::list_directory(&absolute_path, &absolute_path, &options)
            .await
            .map_err(|e: DirectoryError| ClientError::from(e))?;

        Ok(tree)
    }
}