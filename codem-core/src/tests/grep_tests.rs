use tempfile::TempDir;
use std::fs;
use tokio::io::AsyncWriteExt;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grep::grep_codebase;
    use crate::types::GrepOptions;

    #[tokio::test]
    async fn test_grep_file_basic() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");

        let content = "test line\nanother line\nfinal line\n";
        let mut f = tokio::fs::File::create(&file).await.unwrap();
        f.write_all(content.as_bytes()).await.unwrap();

        let pattern = Regex::new("line").unwrap();
        let options = GrepOptions::default();
        let results = grep_codebase(temp.path(), &pattern, &options).await.unwrap();
        
        assert_eq!(results.len(), 1, "Should have found matches in one file");
        let file_match = &results[0];
        assert_eq!(file_match.path, file);
        assert_eq!(file_match.matches.len(), 3, "Should have found three matches");
    }

    #[tokio::test]
    async fn test_grep_file_no_match() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");

        let content = "some content\nno matches here\n";
        let mut f = tokio::fs::File::create(&file).await.unwrap();
        f.write_all(content.as_bytes()).await.unwrap();

        let pattern = Regex::new("line").unwrap();
        let options = GrepOptions::default();
        let results = grep_codebase(temp.path(), &pattern, &options).await.unwrap();
        assert!(results.is_empty(), "Should not have found any matches");
    }

    #[tokio::test]
    async fn test_grep_codebase() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join("subdir")).unwrap();

        // Create multiple files
        let files = vec![
            ("test1.txt", "test line 1\nanother one\n"),
            ("test2.txt", "no matches in this one\n"),
            ("subdir/test3.txt", "test line 3\nline here too\n"),
        ];

        for (name, content) in files {
            let mut f = tokio::fs::File::create(temp.path().join(name)).await.unwrap();
            f.write_all(content.as_bytes()).await.unwrap();
        }

        let options = GrepOptions {
            context_lines: 0,
            file_pattern: None,
            case_sensitive: true,
        };

        let pattern = Regex::new("line").unwrap();
        let results = grep_codebase(temp.path(), &pattern, &options).await.unwrap();

        assert_eq!(results.len(), 2, "Should have found matches in two files");
        assert!(results.iter().any(|m| m.path == temp.path().join("test1.txt")));
        assert!(results.iter().any(|m| m.path == temp.path().join("subdir/test3.txt")));

        let file1_matches = results.iter().find(|m| m.path == temp.path().join("test1.txt")).unwrap();
        assert_eq!(file1_matches.matches.len(), 1);

        let file3_matches = results.iter().find(|m| m.path == temp.path().join("subdir/test3.txt")).unwrap();
        assert_eq!(file3_matches.matches.len(), 2);
    }
}