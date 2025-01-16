use std::path::PathBuf;

use error_set::error_set;

error_set! {
    ConfigError = {
        #[display("Invalid session file path: {}", path.display())]
        InvalidSessionFile { path: PathBuf },
        #[display("Invalid pattern: {}", pattern)]
        InvalidPattern { pattern: String },
        #[display("Invalid project: {}", name)]
        InvalidProject { name: String },
    };
}