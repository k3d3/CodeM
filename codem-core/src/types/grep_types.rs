use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GrepMatch {
    pub line_number: usize,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct GrepFileMatch {
    pub path: PathBuf,
    pub matches: Vec<GrepMatch>,
}

#[derive(Debug, Clone, Default)]
pub struct GrepOptions {
    pub context_lines: usize,
    pub file_pattern: Option<String>,
    pub case_sensitive: bool,
}