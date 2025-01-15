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
        self.sessions.check_timestamp(session_id, path)?;

        let contents = tokio::fs::read_to_string(path)
            .await
            .map_err(ClientError::from)?;

        Ok(contents)
    }
}