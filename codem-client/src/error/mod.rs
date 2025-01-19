pub mod grep_error;
pub mod config_error;
pub use grep_error::GrepError;
pub use config_error::ConfigError;

use std::{path::PathBuf, time::SystemTime};
use error_set::error_set;
use codem_core::error::{WriteError, CommandError};

error_set! {
    ClientError = {
        #[display("File not found: {}", path.display())]
        FileNotFound { 
            path: PathBuf,
        },
        #[display("IO error: {0}")]
        IoError(std::io::Error),
        #[display("Could not create directory {}", path.display())]
        DirCreateError { path: PathBuf },
        #[display("Write error: {0}")]
        WriteError(WriteError),
        #[display("Command error: {0}")]
        CommandError(CommandError),
        #[display("Session not found: {id}")]
        SessionNotFound { id: String },
        #[display("Attempted to write to a file was not previously read in this session")]
        FileNotSynced {
            content: Option<String>,
        },
        #[display("Path not in project scope: {}", path.display())]
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
        #[display("Path not allowed: {}", path.display())]
        PathNotAllowed { 
            path: PathBuf,
        },
        #[display("Invalid path: {}", path.display())]
        InvalidPath { 
            path: PathBuf,
        },
        #[display("Path exists: {}", path.display())]
        PathExists { 
            path: PathBuf,
            content: Option<String>,
        },
        #[display("Permission denied: {}", path.display())]
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
        #[display("Test command failed: {message}")]
        TestCommandFailed { message: String },
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
