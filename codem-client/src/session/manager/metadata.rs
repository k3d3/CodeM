use std::{path::Path, time::SystemTime, sync::Arc};
use crate::{
    error::{ClientError, OperationError},
    session::SessionManager,
};

impl SessionManager {
    pub fn update_timestamp(
        &self,
        session_id: &str,
        path: &Path,
        timestamp: SystemTime,
    ) -> Result<(), ClientError> {
        let session = self.get_session(session_id)?;
        let mut sessions = self.sessions.write();

        if let Some(info) = sessions.get_mut(&session.id) {
            let mut info = (**info).clone();
            info.file_timestamps.insert(path.to_path_buf(), timestamp);
            sessions.insert(session.id.clone(), Arc::new(info));
        }

        Ok(())
    }

    pub fn get_timestamp(
        &self,
        session_id: &str,
        path: &Path,
    ) -> Result<SystemTime, ClientError> {
        let session = self.get_session(session_id)?;

        if let Some(stored) = session.file_timestamps.get(path) {
            Ok(*stored)
        } else {
            let metadata = path.metadata().map_err(OperationError::IoError)?;
            let modified = metadata.modified().map_err(OperationError::IoError)?;
            Ok(modified)
        }
    }

    pub fn check_timestamp(&self, session_id: &str, path: &Path) -> Result<(), ClientError> {
        let session = self.get_session(session_id)?;

        if let Some(stored) = session.file_timestamps.get(path) {
            let metadata = path.metadata().map_err(OperationError::IoError)?;
            let current = metadata.modified().map_err(OperationError::IoError)?;

            if current != *stored {
                return Err(OperationError::TimestampMismatch {
                    path: path.to_string_lossy().into_owned(),
                }
                .into());
            }
        }

        Ok(())
    }
}