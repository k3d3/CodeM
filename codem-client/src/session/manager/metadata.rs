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

#[derive(Clone)]
pub struct Metadata {
    file: PathBuf,
    pub(crate) timestamps: HashMap<String, HashMap<PathBuf, SystemTime>>,
}

impl Metadata {
    pub async fn new(file: PathBuf) -> Self {
        let timestamps = Self::load_file(&file).await.unwrap_or_default();
        Self { file, timestamps }
    }

    async fn load_file(path: &Path) -> io::Result<HashMap<String, HashMap<PathBuf, SystemTime>>> {
        let contents = match fs::read_to_string(path).await {
            Ok(c) => c,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(HashMap::new()),
            Err(e) => return Err(e),
        };

        let serialized: HashMap<String, HashMap<String, SerializableTimestamp>> = 
            toml::from_str(&contents).unwrap_or_default();

        // Convert into our desired type
        let timestamps = serialized.into_iter()
            .map(|(session_id, timestamps)| {
                let converted_timestamps = timestamps.into_iter()
                    .map(|(path, timestamp)| {
                        (PathBuf::from(path), SystemTime::from(timestamp))
                    })
                    .collect();
                (session_id, converted_timestamps)
            })
            .collect();

        Ok(timestamps)
    }

    async fn save_file(&self) -> Result<(), ClientError> {
        // Convert to serializable format
        let serializable: HashMap<String, HashMap<String, SerializableTimestamp>> = self.timestamps
            .iter()
            .map(|(session_id, timestamps)| {
                let converted_timestamps = timestamps.iter()
                    .map(|(path, time)| {
                        (path.to_string_lossy().into_owned(), (*time).into())
                    })
                    .collect();
                (session_id.clone(), converted_timestamps)
            })
            .collect();

        let toml_str = toml::to_string(&serializable)?;
        fs::write(&self.file, toml_str).await?;
        Ok(())
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

    pub async fn update_timestamp(&mut self, session_id: &str, path: &Path, timestamp: SystemTime) -> Result<(), ClientError> {
        let session_stamps = self.timestamps
            .entry(session_id.to_string())
            .or_default();

        session_stamps.insert(path.to_path_buf(), timestamp);
        self.save_file().await?;

        Ok(())
    }
}
