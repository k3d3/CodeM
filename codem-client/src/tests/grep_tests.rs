use crate::error::GrepError;
use crate::client::Client;
use crate::client::grep::{grep_file, grep_codebase};

use rstest::*;
use tempfile::tempdir;
use std::fs;

#[rstest]
#[tokio::test]
async fn test_grep_file_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "test line 1\nfound this\ntest line 3").unwrap();

    let matches = grep_file(&file_path, "found").await.unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].line_number, 2);
    assert_eq!(matches[0].line, "found this");
}

#[rstest]
#[tokio::test]
async fn test_grep_file_not_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nonexistent.txt");

    let result = grep_file(&file_path, "pattern").await;
    assert!(matches!(result,
        Err(GrepError::FileNotFound { path }) if path.contains("nonexistent.txt")
    ));
}

#[rstest]
#[tokio::test]
async fn test_grep_file_invalid_pattern() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "test line").unwrap();

    let result = grep_file(&file_path, "*invalid)").await;
    assert!(matches!(result,
        Err(GrepError::InvalidPattern(_))
    ));
}

#[rstest]
#[tokio::test]
async fn test_grep_codebase() {
    let dir = tempdir().unwrap();
    
    fs::create_dir(dir.path().join("subdir")).unwrap();
    fs::write(
        dir.path().join("file1.txt"),
        "test line 1\nfound this\ntest line 3"
    ).unwrap();
    fs::write(
        dir.path().join("subdir/file2.txt"), 
        "another line\nfound that\nlast line"
    ).unwrap();

    let results = grep_codebase(dir.path(), "found").await.unwrap();
    assert_eq!(results.matches.len(), 2);
    assert_eq!(results.files_searched, 2);
    assert!(results.lines_searched >= 6);

    let mut matches = results.matches;
    matches.sort_by_key(|m| m.file_path.clone());

    assert_eq!(matches[0].line_number, 2);
    assert_eq!(matches[0].line, "found this");
    assert_eq!(matches[1].line_number, 2);
    assert_eq!(matches[1].line, "found that");
}

#[rstest]
#[tokio::test]
async fn test_grep_codebase_skip_binary() {
    let dir = tempdir().unwrap();
    
    fs::write(
        dir.path().join("test.txt"),
        "found this"
    ).unwrap();
    fs::write(
        dir.path().join("test.exe"), 
        "found that"
    ).unwrap();

    let results = grep_codebase(dir.path(), "found").await.unwrap();
    assert_eq!(results.matches.len(), 1);
    assert_eq!(results.files_searched, 1);
    assert!(results.matches[0].line == "found this");
}

#[rstest]
#[tokio::test]
async fn test_grep_integration() {
    let client = Client::new();
    let dir = tempdir().unwrap();
    
    fs::write(
        dir.path().join("test.txt"),
        "line one\nfind me\nline three"
    ).unwrap();

    let matches = client.grep_file(
        dir.path().join("test.txt"), 
        "find"
    ).await.unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].line, "find me");
    assert_eq!(matches[0].line_number, 2);

    let results = client.grep_codebase(dir.path(), "find").await.unwrap();
    assert_eq!(results.matches.len(), 1);
    assert_eq!(results.files_searched, 1);
    assert_eq!(results.lines_searched, 3);
    assert_eq!(results.pattern, "find");
}