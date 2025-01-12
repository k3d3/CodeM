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
    pub include_hidden: bool,
    pub include_size: bool,
    pub count_lines: bool,
    pub file_pattern: Option<String>,
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
