use std::path::Path;
use codem_core::{
    types::{WriteOperation, WriteResult, PartialWrite, PartialWriteLarge, Change},
    fs_ops::ReadOptions,
}; 
use crate::{Client, error::ClientError};

pub(crate) mod operations;

impl Client {
    pub async fn write_file_full(
        &self,
        session_id: &str,
        path: &Path,
        contents: &str,
        run_test: bool,
    ) -> Result<WriteResult, ClientError> {
        // Get session to access project
        let session = self.sessions.get_session(session_id).await?;
        
        // Resolve path relative to project base path
        let absolute_path = session.project.base_path.join(path);
        
        // Validate the path
        self.sessions.check_path(session_id, &absolute_path).await?;

        // Get stored timestamp
        let stored_timestamp = self.sessions.get_timestamp(session_id, &absolute_path).await?;

        // Get current file state
        let read_result = codem_core::fs_read::read_file(&absolute_path, ReadOptions::default()).await;
        
        // Handle file errors
        let (current_content, metadata) = match read_result {
            Ok(result) => result,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(ClientError::FileNotFound { 
                    path: absolute_path
                });
            }
            Err(e) => return Err(ClientError::IoError(e))
        };

        // Get current timestamp if available
        let current_timestamp = metadata.modified.ok_or_else(|| ClientError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, "Could not get file timestamp")
        ))?;

        // Verify timestamps match
        if stored_timestamp != current_timestamp {
            // Update timestamp since we just read the file
            if let Ok(()) = self.sessions.update_timestamp(session_id, &absolute_path, current_timestamp).await {
                return Err(ClientError::FileModifiedSinceRead {
                    content: Some(current_content)
                });
            }
        }

        let result = operations::handle_operation(
            self,
            session_id,
            &absolute_path,
            WriteOperation::Full(contents.to_string()),
            run_test
        ).await?;

        // Update timestamp after successful write
        self.sessions.update_timestamp(session_id, &absolute_path, result.modified).await?;

        Ok(result)
    }

    pub async fn write_file_partial(
        &self,
        session_id: &str,
        path: &Path, 
        changes: Vec<Change>,
        run_test: bool,
    ) -> Result<WriteResult, ClientError> {
        // Get session to access project
        let session = self.sessions.get_session(session_id).await?;
        
        // Resolve path relative to project base path
        let absolute_path = session.project.base_path.join(path);
        
        // Validate the path
        self.sessions.check_path(session_id, &absolute_path).await?;

        // First check if the file exists
        if !absolute_path.exists() {
            return Err(ClientError::FileNotFound {
                path: absolute_path
            });
        }
        
        // Get stored timestamp
        let stored_timestamp = self.sessions.get_timestamp(session_id, &absolute_path).await?;
        
        // Get current file state
        let read_result = codem_core::fs_read::read_file(&absolute_path, ReadOptions::default()).await;
        
        // Handle file errors
        let (current_content, metadata) = match read_result {
            Ok(result) => result,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(ClientError::FileNotFound {
                    path: absolute_path
                });
            }
            Err(e) => return Err(ClientError::IoError(e))
        };

        // Get current timestamp if available
        let current_timestamp = metadata.modified.ok_or_else(|| ClientError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, "Could not get file timestamp")
        ))?;

        // Verify timestamps match
        if stored_timestamp != current_timestamp {
            // Update timestamp since we just read the file
            self.sessions.update_timestamp(session_id, &absolute_path, current_timestamp).await?;

            return Err(ClientError::FileNotSynced {
                content: Some(current_content)
            });
        }

        let partial = PartialWrite {
            context_lines: 3,
            return_full_content: true,
            changes,
            line_range: None,
        };

        let result = operations::handle_operation(
            self,
            session_id,
            &absolute_path,
            WriteOperation::Partial(partial),
            run_test
        ).await?;

        // Update timestamp after successful write
        self.sessions.update_timestamp(session_id, &absolute_path, result.modified).await?;

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
        // Get session to access project
        let session = self.sessions.get_session(session_id).await?;
        
        // Resolve path relative to project base path
        let absolute_path = session.project.base_path.join(path);
        
        // Validate the path
        self.sessions.check_path(session_id, &absolute_path).await?;

        // First check if the file exists
        if !absolute_path.exists() {
            return Err(ClientError::FileNotFound {
                path: absolute_path
            });
        }
        
        // Get stored timestamp
        let stored_timestamp = self.sessions.get_timestamp(session_id, &absolute_path).await?;
        
        // Get current file state
        let read_result = codem_core::fs_read::read_file(&absolute_path, ReadOptions::default()).await;
        
        // Handle file errors
        let (current_content, metadata) = match read_result {
            Ok(result) => result,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(ClientError::FileNotFound {
                    path: absolute_path
                });
            }
            Err(e) => return Err(ClientError::IoError(e))
        };

        // Get current timestamp if available
        let current_timestamp = metadata.modified.ok_or_else(|| ClientError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, "Could not get file timestamp")
        ))?;

        // Verify timestamps match
        if stored_timestamp != current_timestamp {
            // Update timestamp since we just read the file
            self.sessions.update_timestamp(session_id, &absolute_path, current_timestamp).await?;

            return Err(ClientError::FileNotSynced {
                content: Some(current_content)
            });
        }

        let partial = PartialWriteLarge {
            start_str: start_str.to_string(),
            end_str: end_str.to_string(),
            new_str: new_str.to_string(),
            context_lines: 3,
            line_range: None,
        };

        let result = operations::handle_operation(
            self, 
            session_id,
            &absolute_path,
            WriteOperation::PartialLarge(partial),
            run_test
        ).await?;

        // Update timestamp after successful write
        self.sessions.update_timestamp(session_id, &absolute_path, result.modified).await?;

        Ok(result)
    }
}