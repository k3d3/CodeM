// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Session {
//     id: SessionId,
//     project_name: String,
//     allowed_paths: Vec<PathBuf>,
//     file_timestamps: HashMap<PathBuf, SystemTime>,
// }

// impl Session {
//     pub fn new(project_name: String, session_id: SessionId) -> Self {
//         Self {
//             id: session_id,
//             project_name,
//             file_timestamps: HashMap::new(),
//         }
//     }

//     pub fn id(&self) -> &SessionId {
//         &self.id
//     }

//     pub fn project_name(&self) -> &str {
//         &self.project_name
//     }

//     pub fn allowed_paths(&self) -> &[PathBuf] {
//         &self.allowed_paths
//     }

//     pub fn update_timestamp(&mut self, path: PathBuf, timestamp: SystemTime) {
//         // Ensure we have a canonical path
//         if let Ok(canonical) = path.canonicalize() {
//             println!("Updating timestamp for canonical path: {:?}", canonical);
//             self.file_timestamps.insert(canonical, timestamp);
//         } else {
//             println!("Updating timestamp for raw path: {:?}", path);
//             self.file_timestamps.insert(path, timestamp);
//         }
//         println!("Current timestamps: {:?}", self.file_timestamps);
//     }

//     pub fn check_timestamp(&self, path: &Path) -> bool {
//         // Only check timestamps for files that exist
//         if !path.exists() {
//             println!("Path does not exist: {:?}", path);
//             return true;
//         }

//         // Get canonical path if possible
//         let check_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
//         println!("Checking timestamp for path: {:?}", check_path);
//         println!("Current timestamps: {:?}", self.file_timestamps);

//         let exists = self.file_timestamps.contains_key(&check_path);
//         println!("Timestamp exists: {}", exists);
//         exists
//     }
// }
