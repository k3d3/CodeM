use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::SystemTime,
};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    error::{
        CheckTimestampError, CreateSessionError, LoadError, ProjectError, SessionError,
        SessionIdError, SessionSaveError,
    },
    project::{Project, Projects},
};

// 10 neutral (not-negative) adjectives, 10 neutral nouns
// that would likely be whole tokens in common LLMs
const ADJECTIVES: [&str; 10] = [
    "able", "big", "blue", "bold", "bright", "clean", "clear", "close", "cold", "cool",
];

const NOUNS: [&str; 10] = [
    "apple", "bat", "bear", "bed", "bell", "bird", "boat", "book", "boot", "cake",
];

/// Session ID to be used with CodeM.
/// Each session ID is two words separated by a hyphen.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

impl SessionId {
    // Generate a new session ID
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let adj = ADJECTIVES.choose(&mut rng).unwrap();
        let noun = NOUNS.choose(&mut rng).unwrap();
        Self(format!("{}-{}", adj, noun))
    }

    // Generate a new session ID, ensuring it is unique
    pub fn new_unique(existing: &[SessionId]) -> Result<Self, SessionIdError> {
        for _ in 0..50 {
            let id = Self::new_random();

            if !existing.contains(&id) {
                return Ok(id);
            }
        }

        Err(SessionIdError::TooManyAttempts)
    }
}

impl TryFrom<String> for SessionId {
    type Error = SessionIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 2 {
            return Err(SessionIdError::InvalidSessionId);
        }

        if !ADJECTIVES.contains(&parts[0]) {
            return Err(SessionIdError::InvalidSessionId);
        }
        if !NOUNS.contains(&parts[1]) {
            return Err(SessionIdError::InvalidSessionId);
        }

        Ok(Self(value))
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: SessionId,
    project: Project,
    file_timestamps: HashMap<PathBuf, SystemTime>,
}

impl Session {
    pub fn new(project: Project, session_id: SessionId) -> Result<Self, SessionError> {
        let session = Self {
            id: session_id,
            project,
            file_timestamps: HashMap::new(),
        };

        Ok(session)
    }

    pub fn update_timestamp(&mut self, path: &Path, timestamp: SystemTime) {
        self.file_timestamps.insert(path.to_path_buf(), timestamp);
    }

    pub fn get_timestamp(&self, path: &Path) -> Option<SystemTime> {
        self.file_timestamps.get(path).cloned()
    }

    /// Check to make sure the timestamp on disk matches our timestamp
    /// This is in order to make sure we're not working with stale data
    /// If we haven't accessed the file before, return false
    /// If the file doesn't exist, return true
    pub fn check_timestamp(&self, path: &Path) -> Result<bool, CheckTimestampError> {
        let Some(last_seen) = self.file_timestamps.get(path) else {
            return Ok(false);
        };

        let metadata =
            match codem_core::get_metadata(path, codem_core::ReadOptions { count_lines: false }) {
                Ok(metadata) => metadata,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        return Ok(true);
                    } else {
                        return Err(CheckTimestampError::IoError(e));
                    }
                }
            };

        Ok(metadata.modified == *last_seen)
    }

    pub fn path_allowed(&self, path: &Path) -> bool {
        self.project.path_allowed(path)
    }

    pub fn get_base_path(&self) -> &Path {
        self.project.get_base_path()
    }
}

pub struct Sessions {
    projects: Projects,
    sessions: HashMap<SessionId, Mutex<Session>>,
    persist_path: PathBuf,
}

impl Sessions {
    pub async fn new(
        projects: Projects,
        persist_path: impl AsRef<Path>,
    ) -> Result<Self, SessionError> {
        let sessions = Self::load(persist_path.as_ref())
            .await
            .map_err(SessionError::from)?
            .into_iter()
            .map(|(k, v)| (k, Mutex::new(v)))
            .collect();
        let persist_path = persist_path.as_ref().to_path_buf();
        Ok(Self {
            projects,
            sessions,
            persist_path,
        })
    }

    pub async fn create_session(
        &mut self,
        project_name: &str,
    ) -> Result<SessionId, CreateSessionError> {
        let existing_sessions = self.sessions.keys().cloned().collect::<Vec<_>>();
        let session_id = SessionId::new_unique(&existing_sessions)?;
        let project = self
            .projects
            .get(project_name)
            .ok_or(ProjectError::NotFound)?;

        let session = Session::new(project.clone(), session_id.clone())?;
        self.sessions
            .insert(session_id.clone(), Mutex::new(session));

        Ok(session_id)
    }

    pub async fn get(&self, session_id: &SessionId) -> Option<MutexGuard<Session>> {
        Some(self.sessions.get(session_id)?.lock().await)
    }

    pub async fn remove(&mut self, session_id: &SessionId) -> Result<bool, SessionSaveError> {
        let session = self.sessions.remove(session_id).is_some();
        self.save().await;
        Ok(session)
    }

    async fn load(path: &Path) -> Result<HashMap<SessionId, Session>, LoadError> {
        let contents = std::fs::read_to_string(path)?;
        let sessions: HashMap<SessionId, Session> = toml::from_str(&contents)?;

        Ok(sessions)
    }

    async fn save(&self) -> Result<(), SessionSaveError> {
        let mut sessions = HashMap::new();
        for (k, v) in self.sessions.iter() {
            sessions.insert(k.clone(), v.lock().await.clone());
        }
        let contents = toml::to_string_pretty(&sessions)?;
        std::fs::write(&self.persist_path, contents)?;

        Ok(())
    }
}
