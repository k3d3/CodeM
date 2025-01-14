mod checks;
mod operations;

use super::Client;
use crate::error::ClientError;
use crate::session::SessionId;
use crate::types::{CheckOptions, WriteOperation, WriteResult};
use std::path::Path;

use checks::run_checks;
use operations::handle_operation;

impl Client {
    /// Write to a file using the specified mode
    pub async fn write_file(
        &self,
        session_id: &SessionId,
        path: &Path,
        operation: WriteOperation,
        options: CheckOptions,
    ) -> Result<WriteResult, ClientError> {
        let mut session = self.get_session(session_id).await?;

        // Check if path is allowed
        if !session.path_allowed(path) {
            return Err(ClientError::PathNotAllowed);
        }

        let timestamp = session.get_timestamp(path);

        codem_core::write_file(path, operation, options).await?;

        // let mut result = handle_operation(path, operation).await?;

        // // Update timestamp after successful write
        // if let Ok(metadata) = path.metadata() {
        //     session.update_timestamp(path.to_path_buf(), metadata.modified()?);
        // }

        // // If write failed but we got original content, update timestamp
        // if result.original_content.is_some() {
        //     if let Ok(metadata) = path.metadata() {
        //         session.update_timestamp(path.to_path_buf(), metadata.modified()?);
        //     }
        // }

        // // Run checks if requested
        // run_checks(&mut result, &options);

        // Ok(result)
        todo!()
    }
}
