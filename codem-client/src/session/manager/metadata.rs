use std::path::{PathBuf, Path};
use crate::error::ClientError;
use std::time::SystemTime;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Metadata {
    file: PathBuf,
    pub(crate) timestamps: HashMap<String, HashMap<PathBuf, SystemTime>>,
}

impl Metadata {
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            timestamps: HashMap::new(),
        }
    }

    pub fn get_timestamp(&self, session_id: &str, path: &Path) -> Result<SystemTime, ClientError> {
        let path = path.to_path_buf();
        let session_stamps = self.timestamps.get(session_id)
            .ok_or_else(|| ClientError::SessionNotFound { id: session_id.into() })?;

        session_stamps.get(&path)
            .cloned()
            .ok_or_else(|| ClientError::InvalidPath { path })
    }

    pub fn update_timestamp(&mut self, session_id: &str, path: &Path, timestamp: SystemTime) -> Result<(), ClientError> {
        let session_stamps = self.timestamps
            .entry(session_id.to_string())
            .or_insert_with(HashMap::new);

        session_stamps.insert(path.to_path_buf(), timestamp);

        Ok(())
    }
}