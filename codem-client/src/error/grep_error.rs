use error_set::error_set;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct Pattern(pub String);

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for Pattern {}

error_set! {
    GrepError = {
        #[display("Pattern {0} is invalid")]
        InvalidPattern(Pattern),
        #[display("File {path} not found")]
        FileNotFound { path: String },
        #[display("Directory {path} not found")]
        DirectoryNotFound { path: String },
        #[display("Failed to search: {0}")]
        SearchFailed(Pattern),
        #[display("Failed to read file {path}: {source}")]
        ReadError {
            path: String,
            source: std::io::Error
        },
        #[display("Failed to process file {path}: {source}")]
        ProcessError {
            path: String,
            source: std::io::Error
        }
    };
}