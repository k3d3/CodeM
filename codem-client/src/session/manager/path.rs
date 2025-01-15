use std::path::Path;
use crate::{
    error::{ClientError, OperationError},
    session::SessionManager,
};

impl SessionManager {
    pub fn check_path(&self, session_id: &str, path: &Path) -> Result<(), ClientError> {
        let project = self.get_project(session_id)?;
        let mut allowed = false;

        if let Some(parent) = path.parent() {
            if parent.starts_with(&project.base_path) {
                allowed = true;
            }
        }

        if !allowed {
            if let Some(allowed_paths) = &project.allowed_paths {
                for allowed_path in allowed_paths {
                    if path.starts_with(allowed_path) {
                        allowed = true;
                        break;
                    }
                }
            }
        }

        if !allowed {
            return Err(OperationError::PathNotAllowed {
                path: path.to_string_lossy().into_owned(),
            }
            .into());
        }

        Ok(())
    }
}