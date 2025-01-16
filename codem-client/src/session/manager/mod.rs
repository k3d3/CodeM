mod metadata;
mod path;
mod session;

use std::{collections::HashMap, sync::Arc};
use parking_lot::RwLock;

use crate::project::Project;
use super::{SessionId, SessionInfo};

#[derive(Debug)]
pub struct SessionManager {
    projects: HashMap<String, Arc<Project>>,
    sessions: RwLock<HashMap<SessionId, Arc<SessionInfo>>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self {
            projects: HashMap::new(),
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

#[cfg(test)]
impl SessionManager {
    pub fn new_test() -> Self {
        Self::default()
    }
}