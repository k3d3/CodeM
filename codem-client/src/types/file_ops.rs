use std::path::PathBuf;

#[derive(Debug)]
pub enum WriteMode {
    Full,
    Partial,
}

#[derive(Debug)]
pub enum WriteOperation {
    Full(String),
    Partial { old_str: String, new_str: String },
}

#[derive(Debug, Default)]
pub struct CheckOptions {
    pub run_check: bool,
    pub run_lint: bool,
    pub run_test: bool,
}

#[derive(Debug)]
pub struct FileMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct WriteResult {
    pub matches: Vec<FileMatch>,
    pub check_output: Option<String>,
    pub lint_output: Option<String>,
    pub test_output: Option<String>,
    pub original_content: Option<String>,
}
