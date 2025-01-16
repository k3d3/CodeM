use std::path::Path;
use codem_core::types::TreeEntry;
use crate::{Client, error::ClientError};

impl Client {
    pub async fn list_directory(
        &self,
        session_id: &str,
        path: impl AsRef<Path>,
        options: codem_core::types::ListOptions,
    ) -> Result<TreeEntry, ClientError> {
        self.sessions.check_path(session_id, path.as_ref())?;
        let _ = self.sessions.get_timestamp(session_id, path.as_ref()).await?;

        let tree = codem_core::directory::list_directory(path.as_ref(), path.as_ref(), &options)
            .await
            .map_err(ClientError::IoError)?;

        let metadata = path.as_ref().metadata().map_err(ClientError::IoError)?;
        self.sessions.update_timestamp(
            session_id, 
            path.as_ref(),
            metadata.modified()?
        ).await?;

        Ok(tree)
    }
}