use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct GrepOptions {
    pub pattern: String,
    pub case_sensitive: bool,
    pub context_lines: usize,
    pub file_pattern: Option<String>,
}

#[derive(Debug, Default)]
pub struct GrepMatch {
    pub line_number: usize,
    pub context: String,
}

#[derive(Debug, Default)]
pub struct GrepFileMatch {
    pub path: PathBuf,
    pub matches: Vec<GrepMatch>,
}