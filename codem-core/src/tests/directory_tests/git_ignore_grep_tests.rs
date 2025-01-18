use tempfile::TempDir;
use std::fs;
use tokio::io::AsyncWriteExt;

use crate::grep::grep_codebase;
use crate::types::GrepOptions;
use regex::Regex;

#[tokio::test]
async fn test_git_directory_ignored_in_grep() {
    let temp = TempDir::new().unwrap();
    let git_dir = temp.path().join(".git");
    let git_file = git_dir.join("HEAD");
    let normal_dir = temp.path().join("src");
    let normal_file = normal_dir.join("main.rs");

    // Create test directory structure
    fs::create_dir_all(&git_dir).unwrap();
    fs::create_dir_all(&normal_dir).unwrap();

    // Create test files with the same content
    let test_content = b"test content to match";
    let mut file = tokio::fs::File::create(&git_file).await.unwrap();
    file.write_all(test_content).await.unwrap();

    let mut file = tokio::fs::File::create(&normal_file).await.unwrap();
    file.write_all(test_content).await.unwrap();

    // Test grep functionality
    let pattern = Regex::new("test content").unwrap();
    let options = GrepOptions {
        context_lines: 0,
        file_pattern: None,
        case_sensitive: true,
    };

    let results = grep_codebase(temp.path(), &pattern, &options).await.unwrap();
    
    // Should find content only in normal files, not in .git
    assert_eq!(results.len(), 1, "Should only find content in non-git files");
    assert_eq!(results[0].path, normal_file, "Match should be in the normal file");
}