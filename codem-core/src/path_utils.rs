use std::fs;
use std::path::{Path, PathBuf};

pub struct PathValidator {
    allowed_dirs: Vec<PathBuf>,
}

impl PathValidator {
    pub fn new(allowed_dirs: Vec<PathBuf>) -> Self {
        Self { allowed_dirs }
    }

    pub fn get_allowed_dirs(&self) -> Vec<PathBuf> {
        self.allowed_dirs.clone()
    }

    pub fn is_allowed(&self, path: &Path) -> bool {
        // For paths that don't exist yet, just check if they're under an allowed dir
        if !path.exists() {
            return self.is_under_allowed(path);
        }

        // For existing paths, use canonicalization
        let canonical_path = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(_) => return false,
        };

        self.allowed_dirs.iter().any(|allowed_dir| {
            fs::canonicalize(allowed_dir)
                .map(|canonical_allowed| canonical_path.starts_with(canonical_allowed))
                .unwrap_or(false)
        })
    }

    pub fn is_under_allowed(&self, path: &Path) -> bool {
        self.allowed_dirs
            .iter()
            .any(|allowed_dir| path.starts_with(allowed_dir))
    }
}
