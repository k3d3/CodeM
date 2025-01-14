# codem-client Interface

## Core Type

### CodeClient
```rust
pub struct CodeClient {
    core: CodeManager,
}

impl CodeClient {
    /// Create new CodeClient
    pub fn new() -> Result<Self, Error>;

    /// Start a new session for a project
    pub async fn create_session(&self, project_name: &str) -> Result<String, SessionError>;

    /// Write to a file using the specified mode
    pub async fn write_full(
        &self,
        session_id: &SessionId,
        path: &Path,
        opteration: WriteOperation,
        opts: CheckOptions,
    ) -> Result<WriteResult, WriteFullError>;

    /// Read file
    pub async fn read_file(
        &self,
        session_id: &str,
        path: &Path,
        include_stats: bool
    ) -> Result<ReadResult, ReadFileError>;

    // Pass-through methods with session tracking
    pub async fn create_directory(
        &self, 
        session_id: &str,
        path: &Path, 
        recursive: bool
    ) -> Result<(), DirError>;

    pub async fn move_file(
        &self,
        session_id: &str,
        from: &Path,
        to: &Path,
        opts: WriteOptions,
    ) -> Result<ModifyResult, FileOpError>;

    pub async fn copy_file(
        &self,
        session_id: &str,
        from: &Path,
        to: &Path,
        opts: WriteOptions,
    ) -> Result<ModifyResult, FileOpError>;

    pub async fn move_directory(
        &self,
        session_id: &str,
        from: &Path,
        to: &Path,
        opts: WriteOptions,
    ) -> Result<ModifyResult, DirError>;

    pub async fn get_file_stats(
        &self,
        session_id: &str,
        path: &Path
    ) -> Result<FileStats, ReadFileError>;

    pub async fn list_directory(
        &self,
        session_id: &str,
        root: &Path,
        options: ListOptions
    ) -> Result<Vec<DirEntry>, DirError>;

    pub async fn grep_file(
        &self,
        session_id: &str,
        path: &Path,
        pattern: &Regex
    ) -> Result<Vec<GrepMatch>, GrepError>;

    pub async fn grep_codebase(
        &self,
        session_id: &str,
        root: &Path,
        pattern: &Regex,
        options: GrepOptions
    ) -> Result<Vec<GrepMatch>, GrepError>;

    pub async fn run_command(
        &self,
        session_id: &str,
        command: &str,
        config: Option<CommandConfig>
    ) -> Result<CommandOutput, CommandError>;

    pub async fn run_command_risky(
        &self,
        session_id: &str,
        command: &str,
        config: Option<CommandConfig>
    ) -> Result<CommandOutput, CommandError>;
}

## Additional Types

```rust
pub struct WriteOptions {
    pub allow_multiple: bool,
    pub run_check: bool,
    pub run_lint: bool,
    pub run_test: bool,
    pub custom_command: Option<String>,
}

pub struct WriteResult {
    pub stats: Option<FileStats>,
    pub context: Vec<DiffContext>,
    pub check_output: Option<CommandOutput>,
    pub lint_output: Option<CommandOutput>,
    pub test_output: Option<CommandOutput>,
    pub custom_output: Option<CommandOutput>,
}

pub struct ModifyResult {
    pub check_output: Option<CommandOutput>,
    pub lint_output: Option<CommandOutput>,
    pub test_output: Option<CommandOutput>,
    pub custom_output: Option<CommandOutput>,
}

pub struct ReadResult {
    pub content: String,
    pub stats: Option<FileStats>,
}

pub struct DiffContext {
    pub line_number: usize,
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

// Re-exported from codem-core
pub use codem_core::{
    FileStats,
    CommandOutput,
    CommandConfig,
    DirEntry,
    GrepMatch,
    GrepOptions,
    ListOptions,
    WritePartialError,
    WriteFullError,
    ReadFileError,
    FileOpError,
    DirError,
    GrepError,
    CommandError,
};

pub enum SessionError {
    ProjectNotFound,
    InvalidConfig,
    IoError(std::io::Error),
}
```