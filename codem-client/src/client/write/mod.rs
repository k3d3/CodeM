pub mod checks;
pub mod operations;

use std::path::Path;
use std::sync::Arc;

use crate::{
    error::ClientError,
    session::SessionManager,
    types::file_ops::{WriteOperation, WriteResultWithChecks},
    project::Project,
};

pub(super) struct WriteClient {
    sessions: Arc<SessionManager>,
}

impl WriteClient {
    pub fn new(sessions: Arc<SessionManager>) -> Self {
        Self { sessions }
    }

    pub async fn write_file(
        &self,
        session_id: &str,
        path: &Path,
        operation: WriteOperation,
    ) -> Result<WriteResultWithChecks, ClientError> {
        let project = self.sessions.get_project(session_id)?;
        let write_result = operations::handle_operation(path, operation).await?;
        
        let mut result = WriteResultWithChecks {
            line_count: write_result.line_count,
            size: write_result.size,
            details: write_result.details,
            check_results: vec![],
        };

        checks::run_checks(&mut result, &project, path).await?;
        Ok(result)
    }
}