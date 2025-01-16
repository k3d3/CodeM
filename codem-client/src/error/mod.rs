pub mod grep_error;
pub mod config_error;
pub use grep_error::GrepError;
pub use config_error::ConfigError;

use std::path::PathBuf;
use error_set::error_set;
use codem_core::error::{WriteError, CommandError};

error_set! {
    ClientError = {
        #[display("File not found: {}", path.display())]
        FileNotFound { path: PathBuf },
        #[display("IO error: {0}")]
        IoError(std::io::Error),
        #[display("Could not create directory {}", path.display())]
        DirCreateError { path: PathBuf },
        #[display("Write error: {0}")]
        WriteError(WriteError),
        #[display("Command error: {0}")]
        CommandError(CommandError),
        #[display("Session not found: {id}")]
        SessionNotFound { id: Box<str> },
        #[display("Path not in project scope: {}", path.display())]
        PathOutOfScope { path: PathBuf },
        #[display("File timestamp mismatch for {}", path.display())]
        TimestampMismatch { path: PathBuf },
        #[display("Project not found: {name}")]
        ProjectNotFound { name: String },
        #[display("Invalid session ID: {name}")]
        InvalidSessionId{ name: String },
        #[display("File not found: {}", path.display())]
        FileNotReadable { path: PathBuf },
        #[display("File not read: {}", path.display())] 
        FileNotRead { path: PathBuf },
        #[display("Path not allowed: {}", path.display())]
        PathNotAllowed { path: PathBuf },
        #[display("Invalid path: {}", path.display())]
        InvalidPath { path: PathBuf },
        #[display("Path exists: {}", path.display())]
        PathExists { path: PathBuf },
        #[display("Permission denied: {}", path.display())]
        PermissionDenied { path: PathBuf },
        #[display("Config error: {0}")]
        ConfigError(ConfigError),
        #[display("Grep error: {0}")]
        GrepError(GrepError)
    };
}