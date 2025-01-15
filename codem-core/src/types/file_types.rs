use std::time::SystemTime;

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
    PartialLarge(PartialWriteLarge),
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
pub struct PartialWriteLarge {
    pub start_str: String,
    pub end_str: String,
    pub new_str: String,
    pub context_lines: usize,
}

#[derive(Debug)]
pub struct WriteResult {
    pub line_count: usize,
    pub size: usize,
    pub partial_write_result: Option<PartialWriteResult>,
    pub partial_write_large_result: Option<PartialWriteLargeResult>,
}

#[derive(Debug)]
pub struct PartialWriteResult {
    pub change_results: Vec<ChangeResult>,
    pub full_content: Option<String>,
}

#[derive(Debug)]
pub struct PartialWriteLargeResult {
    pub line_number_start: usize,
    pub line_number_end: usize,
    pub context: LargeChangeContext,
}

#[derive(Debug)]
pub struct LargeChangeContext {
    pub before_start: Vec<String>,
    pub start_content: Vec<String>,
    pub end_content: Vec<String>,
    pub after_end: Vec<String>,
}

#[derive(Debug)]
pub struct ChangeResult {
    pub partial_write_index: usize,
    pub line_number_start: usize,
    pub line_number_end: usize,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct MatchInfo {
    pub pattern_index: usize,
    pub relative_match_start: usize,
}