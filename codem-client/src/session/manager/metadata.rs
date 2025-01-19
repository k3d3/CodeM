use std::path::{PathBuf, Path};
use tokio::fs;
use std::io;
use crate::error::ClientError;
use std::time::SystemTime;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SerializableTimestamp(i64);

impl From<SystemTime> for SerializableTimestamp {
    fn from(time: SystemTime) -> Self {
        let duration = time.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        SerializableTimestamp(duration.as_secs() as i64)
    }
}

impl From<SerializableTimestamp> for SystemTime {
    fn from(timestamp: SerializableTimestamp) -> Self {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp.0 as u64)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SessionData {
    project: String,
    files: HashMap<String, i64>,
}

#[derive(Clone)]
pub struct Metadata {
    file: PathBuf,
    pub(crate) timestamps: HashMap<String, HashMap<PathBuf, SystemTime>>,
    pub(crate) projects: HashMap<String, String>, // session_id -> project_name
}

impl Metadata {
    pub async fn new(file: PathBuf) -> Self {
        tracing::info!("Loading session file: {}", file.display());
        let (timestamps, projects) = Self::load_file(&file).await.unwrap_or_default();
        tracing::info!("Loaded sessions: {:?}", projects.keys().collect::<Vec<_>>());
        Self { file, timestamps, projects }
    }

    async fn load_file(path: &Path) -> io::Result<(HashMap<String, HashMap<PathBuf, SystemTime>>, HashMap<String, String>)> {
        let contents = match fs::read_to_string(path).await {
            Ok(c) => {
                tracing::info!("Successfully read session file");
                c
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                tracing::warn!("Session file not found: {}", path.display());
                return Ok((HashMap::new(), HashMap::new()))
            },
            Err(e) => {
                tracing::error!("Failed to read session file: {}", e);
                return Err(e);
            },
        };

        tracing::info!("Parsing TOML file");
        let toml_str = match toml::from_str::<toml::Table>(&contents) {
            Ok(t) => {
                tracing::info!("Session keys found: {:?}", t.keys().collect::<Vec<_>>());
                t
            },
            Err(e) => {
                tracing::error!("Failed to parse session TOML: {}\nContents: {}", e, contents);
                return Ok((HashMap::new(), HashMap::new()));
            }
        };

        // Convert into our desired types
        let mut timestamps = HashMap::new();
        let mut projects = HashMap::new();

        for (session_id, data) in toml_str {
            if let Some(data) = data.as_table() {
                tracing::info!("Processing session: {}", session_id);
                
                // Get project name
                if let Some(project) = data.get("project").and_then(|v| v.as_str()) {
                    tracing::info!("Found project name for session {}: {}", session_id, project);
                    projects.insert(session_id.clone(), project.to_string());
                } else {
                    tracing::warn!("No project found for session {}", session_id);
                }

                // Get files 
                if let Some(files) = data.get("files").and_then(|v| v.as_table()) {
                    let session_files: HashMap<PathBuf, SystemTime> = files.iter()
                        .filter_map(|(path, timestamp)| {
                            timestamp.as_integer().map(|t| {
                                (
                                    PathBuf::from(path),
                                    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(t as u64)
                                )
                            })
                        })
                        .collect();
                    tracing::info!("Found {} files for session {}", session_files.len(), session_id);
                    timestamps.insert(session_id.clone(), session_files);
                } else {
                    tracing::warn!("No files found for session {}", session_id);
                }
            } else {
                tracing::warn!("Session {} data is not a table", session_id);
            }
        }

        tracing::info!("Finished loading {} sessions", projects.len());
        Ok((timestamps, projects))
    }

    async fn save_file(&self) -> Result<(), ClientError> {
        // Convert to serializable format
        let mut root = toml::map::Map::new();

        for (session_id, timestamps) in &self.timestamps {
            let project = self.projects.get(session_id)
                .ok_or_else(|| ClientError::SessionNotFound { id: session_id.to_string() })?;

            let mut session_table = toml::map::Map::new();
            session_table.insert("project".to_string(), toml::Value::String(project.clone()));

            let files: HashMap<String, i64> = timestamps.iter()
                .map(|(path, time)| {
                    let timestamp = time.duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64;
                    (path.to_string_lossy().into_owned(), timestamp)
                })
                .collect();

            session_table.insert("files".to_string(), toml::Value::try_from(files)?);
            root.insert(session_id.clone(), toml::Value::Table(session_table));
        }

        let toml_str = toml::to_string_pretty(&toml::Value::Table(root))?;
        fs::write(&self.file, toml_str).await?;
        Ok(())
    }

    pub fn get_session_project(&self, session_id: &str) -> Option<String> {
        let project = self.projects.get(session_id).cloned();
        if project.is_none() {
            tracing::warn!("No project found for session {}", session_id);
        }
        project
    }

    pub fn get_session_ids(&self) -> Vec<String> {
        self.projects.keys().cloned().collect()
    }

    pub fn get_session_timestamps(&self, session_id: &str) -> HashMap<PathBuf, SystemTime> {
        self.timestamps.get(session_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_timestamp(&self, session_id: &str, path: &Path) -> Result<SystemTime, ClientError> {
        let path = path.to_path_buf();
        let session_stamps = self.timestamps.get(session_id)
            .ok_or_else(|| ClientError::FileNotSynced { content: None })?;

        session_stamps.get(&path)
            .cloned()
            .ok_or_else(|| ClientError::FileNotSynced { 
                content: None 
            })
    }

    pub async fn update_session(&mut self, session_id: &str, project: &str, path: &Path, timestamp: SystemTime) -> Result<(), ClientError> {
        self.projects.insert(session_id.to_string(), project.to_string());
        
        let session_stamps = self.timestamps
            .entry(session_id.to_string())
            .or_default();

        session_stamps.insert(path.to_path_buf(), timestamp);
        self.save_file().await?;

        Ok(())
    }
}