use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrepMatch {
    /// Line number where the match was found
    pub line_number: usize,
    /// The matched line content
    pub line: String,
    /// File path where the match was found (relative to search root)
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrepResults {
    /// Pattern that was searched for
    pub pattern: String,
    /// List of matches found
    pub matches: Vec<GrepMatch>,
    /// Total number of files searched
    pub files_searched: usize,
    /// Total number of lines searched
    pub lines_searched: usize,
}