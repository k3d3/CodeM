# codem-core Interface

## Core Type 

### CodeManager
```rust
pub struct CodeManager {
    allowed_dirs: Vec<PathBuf>,
}

impl CodeManager {
    /// Create new CodeManager with allowed directory paths
    pub fn new(allowed_dirs: Vec<PathBuf>) -> Result<Self, Error>;
    
    /// List all paths this manager has access to
    pub fn list_allowed_paths(&self) -> Vec<PathBuf>;

    /// Replace old_str with new_str in file
    pub async fn write_partial(&self, path: &Path, old_str: &str, new_str: &str) -> Result<(), Error>;
    
    /// Completely replace file contents
    pub async fn write_full(&self, path: &Path, content: &str) -> Result<(), Error>;
    
    /// Read entire file contents
    pub async fn read_file(&self, path: &Path) -> Result<String, Error>;
    
    /// Create a new directory
    pub async fn create_directory(&self, path: &Path) -> Result<(), Error>;

    /// Get stats for a single file
    pub async fn get_file_stats(&self, path: &Path) -> Result<FileStats, Error>;

    /// Search single file for regex pattern
    pub async fn grep_file(&self, path: &Path, pattern: &Regex) -> Result<Vec<GrepMatch>, Error>;

    /// Search codebase for regex pattern with optional filtering
    pub async fn grep_codebase(
        &self,
        root_dir: &Path,
        pattern: &Regex,
        options: GrepOptions
    ) -> Result<Vec<GrepMatch>, Error>;

    /// List directory with optional metadata and filtering
    pub async fn list_directory(
        &self,
        root: &Path,
        options: ListOptions
    ) -> Result<Vec<DirEntry>, Error>;

    /// Run command with optional configuration 
    pub async fn run_command(
        &self,
        command: &str,
        config: Option<CommandConfig>
    ) -> Result<CommandOutput, Error>;
}

## Supporting Types

```rust
pub struct CommandConfig {
    /// Timeout in milliseconds, defaults to 30000
    pub timeout_msecs: u64,
    /// Current working directory for command
    pub cwd: Option<PathBuf>,
    /// Environment variables to set
    pub env: Option<HashMap<String, String>>,
}

pub struct GrepOptions {
    pub file_pattern: Option<String>,
    pub case_sensitive: bool,
}

pub struct GrepMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

pub struct ListOptions {
    pub recursive: bool,
    pub include_size: bool,
    pub include_line_count: bool,
    pub file_pattern: Option<String>,
}

pub struct DirEntry {
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: Option<u64>,
    pub line_count: Option<usize>,
}

pub struct FileStats {
    pub size: u64,
    pub line_count: usize,
}

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub enum Error {
    Io(std::io::Error),
    PatternNotFound,
    MultiplePatternMatches,
    InvalidRegex(regex::Error),
    InvalidGlobPattern,
    PathNotAllowed,
    CommandTimeout(CommandOutput),
    CommandFailed(CommandOutput),
}
```