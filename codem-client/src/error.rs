use error_set::error_set;

error_set! {
    // Top level error aggregating all client errors
    ClientError = OperationError || SessionError || ProjectError;

    OperationError = {
        #[display("Path not allowed: {path}")]
        PathNotAllowed { path: String },
        
        #[display("File has been modified externally: {path}")]
        TimestampMismatch { path: String },
        
        #[display("File must be read before writing: {path}")]
        FileNotRead { path: String },
        
        #[display("File not found: {path}")]
        FileNotFound { path: String },
        
        #[display("Pattern error: {message}")]
        PatternError { message: String },
        
        #[display("IO error: {0}")]
        IoError(std::io::Error),
        
        #[display("Command error: {0}")]
        CommandError(codem_core::error::CommandError),
        
        #[display("Failed to parse TOML: {0}")]
        TomlError(toml::de::Error),
    };

    SessionError = {
        #[display("Session not found: {id}")]
        SessionNotFound { id: String },
        
        #[display("Invalid session ID: {id}")]
        InvalidSessionId { id: String },
        
        #[display("Session already exists: {id}")]
        SessionExists { id: String },
        
        #[display("Could not generate unique session ID after {attempts} attempts")]
        TooManyAttempts { attempts: u32 },
        
        #[display("Failed to save session: {0}")]
        IoError(std::io::Error),
        
        #[display("Failed to serialize session: {0}")]
        SerializationError(toml::ser::Error),
    };

    ProjectError = {
        #[display("Project not found at path: {path}")]
        NotFound { path: String },
        
        #[display("Failed to load project configuration: {0}")]
        IoError(std::io::Error),
        
        #[display("Failed to parse project configuration: {0}")]
        TomlError(toml::de::Error),
    };
}

impl From<codem_core::error::WriteError> for ClientError {
    fn from(err: codem_core::error::WriteError) -> Self {
        use codem_core::error::WriteError;
        match err {
            WriteError::IoError(e) => ClientError::IoError(e),
            WriteError::TimestampMismatch { .. } => ClientError::TimestampMismatch { path: "unknown".to_string() },
            _ => ClientError::PatternError { message: err.to_string() },
        }
    }
}