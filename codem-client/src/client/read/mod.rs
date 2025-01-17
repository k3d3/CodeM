pub mod list;

use std::path::Path;
use codem_core::{fs_ops::ReadOptions, types::FileMetadata};

use crate::{Client, error::ClientError}; 

impl Client {
    pub async fn read_file(
        &self,
        session_id: &str,
        path: &Path,
    ) -> Result<(String, FileMetadata), ClientError> {
        // Get session to access project
        let session = self.sessions.get_session(session_id).await?;
        
        // Resolve path relative to project base path
        let absolute_path = session.project.base_path.join(path);
        
        // Validate the path
        self.sessions.check_path(session_id, &absolute_path).await?;

        // Read the file using codem_core
        let (content, metadata) = codem_core::fs_read::read_file(&absolute_path, ReadOptions { count_lines: true }).await?;

        // Update timestamp after successful read
        if let Some(modified) = metadata.modified {
            self.sessions.update_timestamp(session_id, &absolute_path, modified).await?;
        }

        Ok((content, metadata))
    }
}