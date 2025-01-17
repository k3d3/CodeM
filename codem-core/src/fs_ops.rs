pub use crate::fs_read::{get_metadata, read_file, ReadOptions};
pub use crate::fs_write::write_file;
use std::path::Path;

/// Checks if a given path is inside a .git directory
pub fn is_in_git_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().components().any(|c| c.as_os_str() == ".git")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_in_git_dir() {
        assert!(is_in_git_dir(PathBuf::from(".git/objects")));
        assert!(is_in_git_dir(PathBuf::from("foo/.git/HEAD")));
        assert!(is_in_git_dir(PathBuf::from("foo/bar/.git")));
        assert!(!is_in_git_dir(PathBuf::from("foo/bar")));
        assert!(!is_in_git_dir(PathBuf::from(".gitignore")));
    }
}
