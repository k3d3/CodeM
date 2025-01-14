use crate::directory::list_directory;
use crate::types::ListOptions;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_list_directory() {
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
    assert_eq!(entries[0].path.to_string_lossy(), "file1.txt");
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

    dbg!(&entries);

    assert_eq!(entries.len(), 2);
}
