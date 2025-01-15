use crate::client::Client;
use rstest::*;
use tempfile::{tempdir, TempDir};
use std::fs;
use crate::error::GrepError;

#[fixture]
fn test_dir() -> TempDir {
    let dir = tempdir().unwrap();
    
    fs::write(
        dir.path().join("file1.txt"),
        "test line 1\nfound this\ntest line 3"
    ).unwrap();
    
    fs::create_dir(dir.path().join("subdir")).unwrap();
    fs::write(
        dir.path().join("subdir/file2.txt"),
        "another line\nfound that\nlast line"
    ).unwrap();
    
    fs::write(
        dir.path().join("subdir/binary.exe"),
        "found this but in binary"
    ).unwrap();
    
    dir
}

#[rstest]
#[tokio::test]
async fn test_grep_file_single_match(test_dir: TempDir) {
    let client = Client::new();
    let path = test_dir.path().join("file1.txt");
    
    let result = client.grep_file(&path, "found").await.unwrap();
    
    assert_eq!(result.path, path);
    assert_eq!(result.matches.len(), 1);
    assert_eq!(result.matches[0].line_number, 2);
    assert_eq!(result.matches[0].context, "found this");
}

#[rstest]
#[tokio::test]
async fn test_grep_file_no_match(test_dir: TempDir) {
    let client = Client::new();
    let path = test_dir.path().join("file1.txt");
    
    let result = client.grep_file(&path, "nosuchpattern").await.unwrap();
    
    assert_eq!(result.path, path);
    assert!(result.matches.is_empty());
}

#[rstest]
#[tokio::test]
async fn test_grep_file_not_found(test_dir: TempDir) {
    let client = Client::new();
    let path = test_dir.path().join("nonexistent.txt");
    
    let error = client.grep_file(&path, "pattern").await.unwrap_err();
    assert!(matches!(error,
        GrepError::FileNotFound { path } if path.contains("nonexistent.txt")
    ));
}

#[rstest]
#[tokio::test]
async fn test_grep_file_invalid_pattern(test_dir: TempDir) {
    let client = Client::new();
    let path = test_dir.path().join("file1.txt");
    
    let error = client.grep_file(&path, "*invalid)").await.unwrap_err();
    assert!(matches!(error, GrepError::InvalidPattern(_)));
}

#[rstest]
#[tokio::test]
async fn test_grep_codebase(test_dir: TempDir) {
    let client = Client::new();
    
    let results = client.grep_codebase(test_dir.path(), "found").await.unwrap();
    assert_eq!(results.len(), 2); // Should only match text files
    
    let mut paths: Vec<_> = results.iter()
        .map(|r| &r.path)
        .collect();
    paths.sort();
    
    assert_eq!(paths[0], &test_dir.path().join("file1.txt"));
    let first_match = results.iter()
        .find(|r| r.path == test_dir.path().join("file1.txt"))
        .unwrap();
    assert_eq!(first_match.matches[0].context, "found this");
    
    let second_match = results.iter()
        .find(|r| r.path == test_dir.path().join("subdir/file2.txt"))
        .unwrap();
    assert_eq!(second_match.matches[0].context, "found that");
}

#[rstest]
#[tokio::test]
async fn test_grep_codebase_directory_not_found(test_dir: TempDir) {
    let client = Client::new();
    let invalid_path = test_dir.path().join("nonexistent");
    
    let error = client.grep_codebase(&invalid_path, "test").await.unwrap_err();
    assert!(matches!(error,
        GrepError::DirectoryNotFound { path } if path.contains("nonexistent")
    ));
}

#[rstest]
#[tokio::test]
async fn test_grep_codebase_invalid_pattern(test_dir: TempDir) {
    let client = Client::new();
    
    let error = client.grep_codebase(test_dir.path(), "*invalid)").await.unwrap_err();
    assert!(matches!(error, GrepError::InvalidPattern(_)));
}
