use std::path::Path;
use codem_core::types::{WriteOperation, WriteResult, PartialWrite, PartialWriteLarge, Change}; 
use crate::{Client, error::ClientError};

mod operations;

impl Client {
    pub async fn write_file_full(
        &self,
        session_id: &str,
        path: &Path,
        contents: &str,
        run_test: bool,
    ) -> Result<WriteResult, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let _timestamp = self.sessions.get_timestamp(session_id, path).await?;

        let result = operations::handle_operation(
            self,
            session_id,
            path,
            WriteOperation::Full(contents.to_string()),
            run_test
        ).await?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?).await?;

        Ok(result)
    }

    pub async fn write_file_partial(
        &self,
        session_id: &str,
        path: &Path, 
        changes: Vec<Change>,
        run_test: bool,
    ) -> Result<WriteResult, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let _timestamp = self.sessions.get_timestamp(session_id, path).await?;

        let partial = PartialWrite {
            context_lines: 3,
            return_full_content: true,
            changes,
        };

        let result = operations::handle_operation(
            self,
            session_id,
            path,
            WriteOperation::Partial(partial),
            run_test
        ).await?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?).await?;

        Ok(result)
    }

    pub async fn write_file_large(
        &self,
        session_id: &str,
        path: &Path,
        start_str: &str,
        end_str: &str,
        new_str: &str,
        run_test: bool,
    ) -> Result<WriteResult, ClientError> {
        self.sessions.check_path(session_id, path)?;
        let _timestamp = self.sessions.get_timestamp(session_id, path).await?;

        let partial = PartialWriteLarge {
            start_str: start_str.to_string(),
            end_str: end_str.to_string(),
            new_str: new_str.to_string(),
            context_lines: 3,
        };

        let result = operations::handle_operation(
            self, 
            session_id,
            path,
            WriteOperation::PartialLarge(partial),
            run_test
        ).await?;

        let metadata = path.metadata()?;
        self.sessions.update_timestamp(session_id, path, metadata.modified()?).await?;

        Ok(result)
    }
}