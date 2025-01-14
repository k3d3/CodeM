use error_set::error_set;

error_set! {
    ClientError = SessionError || SessionIdError || CreateSessionError || LoadError || FileError;

    LoadError = {
        #[display("IO error when accessing file")]
        IoError(std::io::Error),
        #[display("TOML error when parsing file")]
        TomlError(toml::de::Error),
    };

    FileError = {
        #[display("Path not allowed")]
        PathNotAllowed,
        #[display("File has been modified externally")]
        FileModified,
        #[display("File must be read before writing")]
        FileNotRead,
        #[display("Pattern not found in file")]
        PatternNotFound,
        #[display("Multiple matches found for pattern")]
        MultipleMatches,
        #[display("Invalid pattern: {pattern}")]
        InvalidPattern {
            pattern: String
        },
        #[display("IO error: {0}")]
        IoError(std::io::Error),
    };

    ProjectError = LoadError || {
        #[display("Project not found")]
        NotFound,
    };

    CheckTimestampError = {
        #[display("IO error when accessing file")]
        IoError(std::io::Error),
    };

    SessionIdError = {
        #[display("Session not found")]
        SessionNotFound,
        #[display("Invalid session ID")]
        InvalidSessionId,
        #[display("Session already exists")]
        SessionExists,
        #[display("Could not generate unique session ID")]
        TooManyAttempts,
    };

    SessionSaveError = {
        #[display("Could not save sessions to file")]
        IoError(std::io::Error),
        #[display("Could not serialize session file")]
        TomlError(toml::ser::Error),
    };

    SessionError = SessionIdError || LoadError || {
        #[display("Could not save sessions to file")]
        SaveError(SessionSaveError),
    };

    CreateSessionError = SessionError || ProjectError;
}
