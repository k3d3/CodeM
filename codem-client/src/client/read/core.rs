use crate::{
    error::Result,
    types::{ReadResult, SessionId},
    Client,
};
use codem_core::types::FileMetadata;
use std::path::Path;

impl Client {
    /// Read contents of a file with optional metadata
    pub async fn read_file(&self, session_id: &SessionId, path: &Path, include_metadata: bool) -> Result<ReadResult> {
        let metadata = if include_metadata && self.is_path_allowed(path) {
            match path.metadata() {
                Ok(m) => Some(FileMetadata {
                    modified: m.modified()?,
                    size: m.len(),
                    line_count: None,
                }),
                Err(_) => None,
            }
        } else {
            None
        };

        let content = std::fs::read_to_string(path)?;
        
        // Update session timestamp
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|s| s.id() == session_id) {
            if let Ok(m) = path.metadata() {
                if let Ok(modified) = m.modified() {
                    session.update_timestamp(path.to_path_buf(), modified);
                }
            }
        }

        Ok(ReadResult { content, metadata })
    }

    /// Get stats for a file
    pub async fn get_file_stats(&self, _session_id: &SessionId, path: &Path) -> Result<FileMetadata> {
        let metadata = path.metadata()?;
        Ok(FileMetadata {
            modified: metadata.modified()?,
            size: metadata.len(),
            line_count: None,
        })
    }
}