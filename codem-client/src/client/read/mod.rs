use std::path::Path;
use crate::{
    Client,
    error::ClientError,
};

impl Client {
    pub async fn read_file(
        &self,
        session_id: &str,
        path: &Path,
    ) -> Result<String, ClientError> {
        self.sessions.check_path(session_id, path)?;

        let (contents, metadata) = codem_core::fs_read::read_file(path, Default::default()).await?;
        
        self.sessions.update_timestamp(session_id, path, metadata.modified.unwrap())?;
        
        Ok(contents)
    }
}