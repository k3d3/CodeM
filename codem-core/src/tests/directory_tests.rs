use crate::directory::list_directory;
use crate::types::ListOptions;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_list_directory_children() {
    let temp = TempDir::new().unwrap();

    fs::write(temp.path().join("file1.txt"), "content1").unwrap();
    fs::write(temp.path().join("file2.txt"), "content2").unwrap();

    let entries = list_directory(temp.path(), temp.path(), &ListOptions::default()).await.unwrap();
    assert_eq!(entries.len(), 2);
}

#[tokio::test]
async fn test_list_directory_pattern() {
    let temp = TempDir::new().unwrap();

    fs::write(temp.path().join("file1.txt"), "content1").unwrap();
    fs::write(temp.path().join("file2.rs"), "content2").unwrap();

    let entries = list_directory(
        temp.path(),
        temp.path(),
        &ListOptions {
            file_pattern: Some(".*\\.txt".into()),
            ..Default::default()
        },
    ).await.unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path().to_string_lossy(), "file1.txt");
}

#[tokio::test]
async fn test_list_directory_recursive() {
    let temp = TempDir::new().unwrap();

    fs::create_dir(temp.path().join("subdir")).unwrap();
    fs::write(temp.path().join("file1.txt"), "content1").unwrap();
    fs::write(temp.path().join("subdir/file2.txt"), "content2").unwrap();

    let entries = list_directory(
        temp.path(),
        temp.path(),
        &ListOptions {
            recursive: true,
            ..Default::default()
        },
    ).await.unwrap();

    assert_eq!(entries.len(), 2);

    // Should have one file and one directory
    let files: Vec<_> = entries.iter().filter(|e| !e.is_dir()).collect();
    let dirs: Vec<_> = entries.iter().filter(|e| e.is_dir()).collect();

    assert_eq!(files.len(), 1);
    assert_eq!(dirs.len(), 1);

    // Check that the directory contains one file
    assert_eq!(dirs[0].len(), 1);
    assert_eq!(dirs[0][0].path().to_string_lossy(), "subdir/file2.txt");
}