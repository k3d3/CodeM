use std::path::Path;

use codem_core::types::FileMetadata;

use crate::error::{ClientError, FileError};
use crate::session::SessionId;
use crate::types::ReadResult;

use super::Client;

impl Client {
    pub async fn read_file(
        &self,
        session_id: &SessionId,
        path: &Path,
    ) -> Result<ReadResult, ClientError> {
        let mut session = self.get_session(session_id).await?;

        // Check if path is allowed
        if !session.path_allowed(path.as_ref()) {
            return Err(ClientError::PathNotAllowed);
        }

        let (content, metadata) =
            codem_core::read_file(path, codem_core::ReadOptions { count_lines: true })?;

        session.update_timestamp(path, metadata.modified);

        Ok(ReadResult { content, metadata })
    }

    pub async fn get_file_metadata(
        &self,
        session_id: &SessionId,
        path: &Path,
    ) -> Result<FileMetadata, ClientError> {
        let session = self.get_session(session_id).await?;

        // Check if path is allowed
        if !session.path_allowed(path.as_ref()) {
            return Err(ClientError::PathNotAllowed);
        }

        codem_core::get_metadata(path, codem_core::ReadOptions { count_lines: true })
            .map_err(|e| FileError::IoError(e).into())
    }
}
