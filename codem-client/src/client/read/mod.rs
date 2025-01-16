pub mod list;

use std::path::Path;
use crate::{Client, error::ClientError}; 

impl Client {
    pub async fn read_file(
        &self,
        session_id: &str,
        path: &Path,
    ) -> Result<String, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let _ = self.sessions.get_timestamp(session_id, path).await?;

        let content = tokio::fs::read_to_string(path).await?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?).await?;

        Ok(content)
    }
}