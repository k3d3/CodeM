use std::path::PathBuf;
use std::time::SystemTime;
use crate::types::FileMetadata;

#[derive(Debug, Default, Clone)]
pub struct ListOptions {
    pub include_size: bool,
    pub include_modified: bool,
    pub file_pattern: Option<String>,
    pub recursive: bool,
    pub count_lines: bool,
}

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub path: PathBuf,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
    pub entry_type: String,
    pub is_dir: bool,
    pub symlink: bool,
    pub stats: Option<FileMetadata>,
}

impl Default for ListEntry {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            size: None,
            modified: None,
            entry_type: "UNK".to_string(),
            is_dir: false,
            symlink: false,
            stats: None,
        }
    }
}