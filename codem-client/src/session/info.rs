use std::{collections::HashMap, path::PathBuf, time::SystemTime};
use super::id::SessionId;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub project_name: String,
    pub id: SessionId,
    pub file_timestamps: HashMap<PathBuf, SystemTime>,
}