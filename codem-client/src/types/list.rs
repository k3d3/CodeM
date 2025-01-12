use std::path::PathBuf;
use codem_core::types::FileMetadata;

#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub recursive: bool,
    pub include_stats: bool,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub stats: Option<FileMetadata>,
}