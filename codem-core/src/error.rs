use std::io;
use std::path::PathBuf;
use thiserror::Error;
use crate::types::CommandOutput;

#[derive(Debug, Error)]
pub enum DirError {
    #[error("Path {0} not found")]
    NotFound(PathBuf),
    
    #[error("Path {0} is not a directory")]
    NotDirectory(PathBuf),
    
    #[error("Path {0} is not within allowed directories")]
    PathNotAllowed(PathBuf),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum WriteFullError {
    #[error("Path {0} is not within allowed directories")]
    PathNotAllowed(PathBuf),

    #[error("File was modified externally")]
    FileModified(String),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum WritePartialError {
    #[error("Path {0} is not within allowed directories")]
    PathNotAllowed(PathBuf),
    
    #[error("Pattern not found")]
    PatternNotFound,
    
    #[error("Multiple pattern matches found: {0}")]
    MultiplePatternMatches(String),

    #[error("File was modified externally: {0}")]
    FileModified(String),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum ReadFileError {
    #[error("Path {0} is not within allowed directories")]
    PathNotAllowed(PathBuf),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("Path {0} is not within allowed directories")]
    PathNotAllowed(PathBuf),
    
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),
    
    #[error("Invalid glob pattern")]
    InvalidGlobPattern,
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command failed with exit code {exit_code}: {stdout} {stderr}")]
    Failed {
        exit_code: i32,
        stdout: String,
        stderr: String,
        output: CommandOutput,
    },

    #[error("Command timed out after {timeout_ms}ms: {stdout} {stderr}")]
    Timeout {
        timeout_ms: u64,
        stdout: String,
        stderr: String,
        output: CommandOutput,
    },

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}