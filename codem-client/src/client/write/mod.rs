mod operations;
mod checks;

use crate::error::{FileError, Result};
use crate::types::{CheckOptions, WriteOperation, WriteResult};
use crate::types::SessionId;
use std::path::Path;
use super::Client;

use operations::handle_operation;
use checks::run_checks;

impl Client {
    /// Write to a file using the specified mode
    pub async fn write(
        &self,
        session_id: &SessionId,
        path: &Path,
        operation: WriteOperation,
        options: CheckOptions,
    ) -> Result<WriteResult> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .iter_mut()
            .find(|s| s.id() == session_id)
            .ok_or(FileError::SessionNotFound)?;

        if !self.is_path_allowed(path) {
            return Err(FileError::PathNotAllowed);
        }

        // Check if file has timestamp
        if !session.check_timestamp(path) {
            return Err(FileError::FileNotRead);
        }

        let mut result = handle_operation(path, operation).await?;

        // Update timestamp after successful write
        if let Ok(metadata) = path.metadata() {
            session.update_timestamp(path.to_path_buf(), metadata.modified()?);
        }

        // If write failed but we got original content, update timestamp
        if result.original_content.is_some() {
            if let Ok(metadata) = path.metadata() {
                session.update_timestamp(path.to_path_buf(), metadata.modified()?);
            }
        }

        // Run checks if requested
        run_checks(&mut result, &options);

        Ok(result)
    }
}