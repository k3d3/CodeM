use std::{path::Path, time::SystemTime, sync::Arc};
use crate::{error::ClientError, session::SessionManager};

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

        session.file_timestamps.get(path)
            .copied()
            .ok_or_else(|| ClientError::FileNotRead {
                path: path.to_path_buf(),
            })
    }

    pub fn check_timestamp(&self, session_id: &str, path: &Path) -> Result<(), ClientError> {
        let session = self.get_session(session_id)?;

        if let Some(stored) = session.file_timestamps.get(path) {
            let metadata = path.metadata().map_err(|e| ClientError::IoError(e))?;
            let current = metadata.modified().map_err(|e| ClientError::IoError(e))?;

            if current != *stored {
                return Err(ClientError::TimestampMismatch {
                    path: path.to_path_buf(),
                });
            }
        }

        Ok(())
    }
}