use crate::types::Session;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SessionStore {
    sessions: HashMap<String, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.insert(session.id().to_string(), session);
    }

    pub fn get_session(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }

    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    pub fn remove_session(&mut self, id: &str) {
        self.sessions.remove(id);
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let contents = toml::to_string_pretty(self).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to serialize: {}", e))
        })?;
        std::fs::write(path, contents)
    }

    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to deserialize: {}", e))
        })
    }
}