use tempfile::TempDir;
use std::fs;
use tokio::io::AsyncWriteExt;

use crate::grep::grep_file;
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
        case_sensitive: true,
        file_pattern: None,
    };

    // Check git file
    let git_result = grep_file(&git_file, &pattern, &options).await.unwrap();
    assert!(git_result.is_none(), "Git file was not ignored in grep");

    // Check normal file
    let normal_result = grep_file(&normal_file, &pattern, &options).await.unwrap();
    assert!(normal_result.is_some(), "Normal file was not found in grep");
}