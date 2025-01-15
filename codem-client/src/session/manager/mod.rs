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