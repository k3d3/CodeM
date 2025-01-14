use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub modified: SystemTime,
    pub size: u64,
    pub line_count: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct CommandConfig {
    pub cwd: Option<PathBuf>,
    pub env: Option<HashMap<String, String>>,
    pub timeout: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Default)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub recursive: bool,
    pub count_lines: bool,
    pub file_pattern: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub symlink: Option<PathBuf>,
    pub stats: Option<FileMetadata>,
}

#[derive(Debug, Clone)]
pub struct FileMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

pub type GrepMatch = FileMatch;

#[derive(Debug, Clone, Default)]
pub struct GrepOptions {
    pub include_hidden: bool,
    pub file_pattern: Option<String>,
}

#[derive(Debug)]
pub enum WriteOperation {
    Full(String),
    Partial(PartialWrite),
}

#[derive(Debug)]
pub struct PartialWrite {
    pub context_lines: usize,
    pub return_full_content: bool,
    pub writes: Vec<PartialWriteInner>,
}

#[derive(Debug)]
pub struct PartialWriteInner {
    pub old_str: String,
    pub new_str: String,
    pub allow_multiple_matches: bool,
}

#[derive(Debug)]
pub struct WriteResult {
    pub line_count: usize,
    pub size: usize,
    pub partial_write_result: Option<PartialWriteResult>
}

#[derive(Debug)]
pub struct PartialWriteResult {
    pub content: Vec<PartialWriteResultContent>,
    pub full_content: Option<String>,
}

#[derive(Debug)]
pub struct PartialWriteResultContent {
    pub partial_write_index: usize,
    pub line_number_start: usize,
    pub line_number_end: usize,
    pub context: String,
}