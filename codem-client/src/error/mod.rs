use std::path::PathBuf;
use error_set::error_set;
pub mod grep_error;
pub use grep_error::GrepError;
use codem_core::error::{WriteError, CommandError};

error_set! {
    ClientError = SessionError || OperationError || {
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
    };

    #[derive(Clone)]
    #[disable(Error)] 
    SessionError = {
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
    };

    OperationError = {
        #[display("File not found: {}", path.display())]
        FileNotFound { path: PathBuf },
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
        #[display("Timestamp mismatch for {}", path.display())]
        TimestampMismatch { path: PathBuf },
        #[display("IO error: {0}")]
        IoError(std::io::Error),
    };
}

impl std::error::Error for SessionError {}