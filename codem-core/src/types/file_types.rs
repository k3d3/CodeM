use std::time::SystemTime;

#[derive(Debug, Clone, Default)]
pub struct FileMetadata {
    pub modified: Option<SystemTime>,
    pub size: Option<u64>,
    pub line_count: Option<usize>,
}

#[derive(Debug)]
pub enum WriteOperation {
    Full(String),
    Partial(PartialWrite),
    PartialLarge(PartialWriteLarge),
}

#[derive(Debug, Clone, Default)]
pub struct LineRange {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

#[derive(Debug, Default)]
pub struct PartialWrite {
    pub context_lines: usize,
    pub return_full_content: bool,
    pub changes: Vec<Change>,
    pub line_range: Option<LineRange>,
}

/// Helper macro for creating Change structs with minimal boilerplate
#[macro_export]
macro_rules! change {
    ($old:expr, $new:expr) => {
        Change {
            old_str: $old.to_string(),
            new_str: $new.to_string(),
            allow_multiple_matches: false,
            line_range: None,
        }
    };
    ($old:expr, $new:expr, allow_multiple: $allow:expr) => {
        Change {
            old_str: $old.to_string(),
            new_str: $new.to_string(),
            allow_multiple_matches: $allow,
            line_range: None,
        }
    };
    ($old:expr, $new:expr, range: $range:expr) => {
        Change {
            old_str: $old.to_string(),
            new_str: $new.to_string(),
            allow_multiple_matches: false,
            line_range: Some($range),
        }
    };
}

#[derive(Debug, Clone)]
pub struct Change {
    pub old_str: String,
    pub new_str: String,
    pub allow_multiple_matches: bool,
    pub line_range: Option<LineRange>,
}

/// Helper macro for creating PartialWriteLarge structs with minimal boilerplate
#[macro_export]
macro_rules! partial_write_large {
    ($start:expr, $end:expr, $new:expr) => {
        PartialWriteLarge {
            start_str: $start.to_string(),
            end_str: $end.to_string(),
            new_str: $new.to_string(),
            context_lines: 2,
            line_range: None,
        }
    };
    ($start:expr, $end:expr, $new:expr, context: $ctx:expr) => {
        PartialWriteLarge {
            start_str: $start.to_string(),
            end_str: $end.to_string(),
            new_str: $new.to_string(),
            context_lines: $ctx,
            line_range: None,
        }
    };
    ($start:expr, $end:expr, $new:expr, range: $range:expr) => {
        PartialWriteLarge {
            start_str: $start.to_string(),
            end_str: $end.to_string(),
            new_str: $new.to_string(),
            context_lines: 2,
            line_range: Some($range),
        }
    };
}

#[derive(Debug, Clone)]
pub struct PartialWriteLarge {
    pub start_str: String,
    pub end_str: String,
    pub new_str: String,
    pub context_lines: usize,
    pub line_range: Option<LineRange>,
}

#[derive(Debug)]
pub enum WriteResultDetails {
    None,
    Partial(PartialWriteResult),
    PartialLarge(PartialWriteLargeResult),
    WithTestOutput {
        output: String,
        details: Box<WriteResultDetails>,
    },
}

#[derive(Debug)]
pub struct WriteResult {
    pub line_count: usize,
    pub size: usize,
    pub modified: SystemTime,
    pub details: WriteResultDetails,
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