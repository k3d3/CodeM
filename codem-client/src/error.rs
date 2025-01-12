#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Path not allowed")]
    PathNotAllowed,
    
    #[error("File has been modified externally")]
    FileModified,
    
    #[error("File must be read before writing")]
    FileNotRead,
    
    #[error("Pattern not found in file")]
    PatternNotFound,
    
    #[error("Multiple matches found for pattern")]
    MultipleMatches,

    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, FileError>;