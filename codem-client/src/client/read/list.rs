use std::path::Path;
use tokio::fs;
use codem_core::types::{TreeEntry, ListOptions};
use super::super::Client;
use codem_core::directory::list_directory as core_list_directory;
use crate::error::ClientError;

impl Client {
    pub async fn list_directory(
        &self,
        session_id: &str, 
        path: impl AsRef<Path>,
        options: ListOptions,
    ) -> Result<TreeEntry, ClientError> {
        let path = path.as_ref();
        self.sessions.check_path(session_id, path)?;
        self.sessions.check_timestamp(session_id, path)?;

        let tree = core_list_directory(path, path, &options).await.map_err(|e| 
            ClientError::IoError(e))?;

        let metadata = fs::metadata(path).await.map_err(|e| 
            ClientError::IoError(e))?;

        self.sessions.update_timestamp(
            session_id, 
            path,
            metadata.modified().unwrap())?;

        Ok(tree)
    }
}