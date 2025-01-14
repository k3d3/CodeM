use std::path::PathBuf;
use std::time::SystemTime;

// File operations types
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub modified: SystemTime,
    pub size: u64,
    pub line_count: Option<usize>,
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
    pub changes: Vec<Change>,
}

#[derive(Debug)]
pub struct Change {
    pub old_str: String,
    pub new_str: String,
    pub allow_multiple_matches: bool,
}

#[derive(Debug)]
pub struct WriteResult {
    pub line_count: usize,
    pub size: usize,
    pub partial_write_result: Option<PartialWriteResult>,
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

#[derive(Debug)]
pub struct MatchInfo {
    pub pattern_index: usize,
    pub relative_match_start: usize,
}

#[derive(Debug)]
pub struct PatternInfo {
    /// Byte size difference between old_str and new_str
    pub size_diff: isize,
    /// Number of lines in new_str
    pub line_count: usize,
}

// Command types
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

// Directory types
#[derive(Debug, Default, Clone)]
pub struct ListOptions {
    pub include_size: bool,
    pub include_modified: bool,
    pub include_type: bool,
    pub file_pattern: Option<String>,
    pub recursive: bool,
    pub count_lines: bool,
}

#[derive(Debug)]
pub struct ListEntry {
    pub path: PathBuf,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
    pub entry_type: Option<String>,
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
            entry_type: None,
            is_dir: false,
            symlink: false,
            stats: None,
        }
    }
}

// Grep types
#[derive(Debug, Clone, Default)]
pub struct GrepOptions {
    pub pattern: String,
    pub case_sensitive: bool,
    pub context_before: usize,
    pub context_after: usize,
    pub file_pattern: Option<String>,
}

#[derive(Debug, Default)]
pub struct GrepMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
    pub line: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}