use std::path::Path;
use codem_core::types::{WriteOperation, WriteResult, PartialWrite, PartialWriteLarge, Change}; 
use crate::{Client, error::ClientError};

impl Client {
    pub async fn write_file_full(
        &self,
        session_id: &str,
        path: &Path,
        contents: &str,
    ) -> Result<WriteResult, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let timestamp = self.sessions.get_timestamp(session_id, path)?;

        let result = codem_core::fs_write::write_file(
            path,
            WriteOperation::Full(contents.to_string()),
            Some(timestamp)
        ).await?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?)?;

        Ok(result)
    }

    pub async fn write_file_partial(
        &self,
        session_id: &str,
        path: &Path, 
        changes: Vec<Change>,
    ) -> Result<WriteResult, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let timestamp = self.sessions.get_timestamp(session_id, path)?;

        let partial = PartialWrite {
            context_lines: 3,
            return_full_content: true,
            changes,
        };

        let result = codem_core::fs_write::write_file(
            path,
            WriteOperation::Partial(partial),
            Some(timestamp)
        ).await.map_err(ClientError::from)?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?)?;

        Ok(result)
    }

    pub async fn write_file_large(
        &self,
        session_id: &str,
        path: &Path,
        start_str: &str,
        end_str: &str,
        new_str: &str,
    ) -> Result<WriteResult, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let timestamp = self.sessions.get_timestamp(session_id, path)?;

        let partial = PartialWriteLarge {
            start_str: start_str.to_string(),
            end_str: end_str.to_string(),
            new_str: new_str.to_string(),
            context_lines: 3,
        };

        let result = codem_core::fs_write::write_file(
            path, 
            WriteOperation::PartialLarge(partial),
            Some(timestamp)
        ).await.map_err(ClientError::from)?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?)?;

        Ok(result)
    }
}