mod grep;

use std::path::Path;
use crate::error::GrepError;
use crate::types::*;
use anyhow::Result;

#[derive(Default)]
pub struct Client {
    // Add client fields as needed
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn read_file(&self, _path: impl AsRef<Path>) -> Result<String> {
        // TODO: Implement read_file
        todo!()
    }

    pub async fn write_file(&self, _path: impl AsRef<Path>, _content: &str) -> Result<()> {
        // TODO: Implement write_file  
        todo!()
    }

    pub async fn grep_file(
        &self, 
        path: impl AsRef<Path>, 
        pattern: &str
    ) -> Result<Vec<GrepMatch>, GrepError> {
        grep::grep_file(path, pattern).await
    }

    pub async fn grep_codebase(
        &self,
        root_dir: impl AsRef<Path>,
        pattern: &str
    ) -> Result<GrepResults, GrepError> {
        grep::grep_codebase(root_dir, pattern).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use tempfile::tempdir;
    use std::fs;

    #[rstest]
    #[tokio::test]
    async fn test_grep_integration() {
        let client = Client::new();
        let dir = tempdir().unwrap();
        
        // Create test files
        fs::write(
            dir.path().join("test.txt"),
            "line one\nfind me\nline three"
        ).unwrap();

        // Test grep_file
        let matches = client.grep_file(
            dir.path().join("test.txt"), 
            "find"
        ).await.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, "find me");
        assert_eq!(matches[0].line_number, 2);

        // Test grep_codebase
        let results = client.grep_codebase(dir.path(), "find").await.unwrap();
        assert_eq!(results.matches.len(), 1);
        assert_eq!(results.files_searched, 1);
        assert_eq!(results.lines_searched, 3);
        assert_eq!(results.pattern, "find");
    }
}