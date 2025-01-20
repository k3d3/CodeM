pub mod grep_error;
pub mod config_error;
pub use grep_error::GrepError;
pub use config_error::ConfigError;

use std::{path::PathBuf, time::SystemTime};
use error_set::error_set;
use codem_core::error::{WriteError, CommandError, DirectoryError};

use crate::session::manager::path::PathValidator;

pub trait ToRelativePath {
    fn to_relative_display(&self, validator: &PathValidator) -> String;
}

impl ToRelativePath for PathBuf {
    fn to_relative_display(&self, validator: &PathValidator) -> String {
        validator.to_relative_path(self).display().to_string()
    }
}

impl From<DirectoryError> for ClientError {
    fn from(err: DirectoryError) -> Self {
        match err {
            DirectoryError::IoError(e) => ClientError::IoError(e),
            DirectoryError::RegexError(_e) => ClientError::InvalidPath { 
                path: PathBuf::from("Invalid regex pattern in file filter") 
            },
        }
    }
}

error_set! {
    ClientError = {
        #[display("File not found: {}", path.to_string_lossy())]
        FileNotFound { 
            path: PathBuf,
        },
        #[display("IO error: {0}")]
        IoError(std::io::Error),
        #[display("Could not create directory {}", path.to_string_lossy())]
        DirCreateError { path: PathBuf },
        #[display("Write error: {0}")]
        WriteError(WriteError),
        #[display("Command error: {0}")]
        CommandError(CommandError),
        #[display("Session not found: {id}")]
        SessionNotFound { id: String },
        #[display("Attempted to write to a file was not previously read in this session. The file contents have now been read and are below, so you can try writing again.")]
        FileNotSynced {
            content: Option<String>,
        },
        #[display("Path not in project scope: {}", path.to_string_lossy())]
        PathOutOfScope { 
            path: PathBuf,
        },
        #[display("File timestamp mismatch for {}", path.display())]
        TimestampMismatch { 
            path: PathBuf,
            current_timestamp: SystemTime,
            expected_timestamp: SystemTime,
            content: String,
        },
        #[display("Project not found: {name}")]
        ProjectNotFound { name: String },
        #[display("Invalid session ID: {name}")]
        InvalidSessionId { name: String },
        #[display("File not readable: {}", path.display())]
        FileNotReadable { 
            path: PathBuf,
            content: Option<String>,
        },
        #[display("File not read: {}", path.display())] 
        FileNotRead { 
            path: PathBuf,
            content: Option<String>,
        },
        #[display("Path not allowed: {}", path.to_string_lossy())]
        PathNotAllowed { 
            path: PathBuf,
        },
        #[display("Invalid path: {}", path.to_string_lossy())]
        InvalidPath { 
            path: PathBuf,
        },
        #[display("Path exists: {}", path.to_string_lossy())]
        PathExists { 
            path: PathBuf,
            content: Option<String>,
        },
        #[display("Permission denied: {}", path.to_string_lossy())]
        PermissionDenied { 
            path: PathBuf,
            content: Option<String>,
        },
        #[display("Config error: {0}")]
        ConfigError(ConfigError),
        #[display("Grep error: {0}")]
        GrepError(GrepError),
        #[display("Test command not configured")]
        TestCommandNotConfigured,
        #[display("Test command failed (exit code {exit_code}):\nstdout:\n{stdout}\nstderr:\n{stderr}")]
        TestCommandFailed {
            stdout: String,
            stderr: String,
            exit_code: i32,
        },
        #[display("Toml deserialize error: {0}")]
        TomlDeserializeError(toml::de::Error),
        #[display("Toml serialize error: {0}")]
        TomlSerializeError(toml::ser::Error),
        #[display("Command not recognized: {command}")]
        InvalidCommand { command: String },
        #[display("Command requires timeout: {command}")]
        UnsafeCommand { command: String },
    };
}