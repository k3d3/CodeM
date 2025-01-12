use std::fs;
use tempfile::TempDir;
use crate::directory::list_directory;
use crate::types::ListOptions;

#[tokio::test]
async fn test_list_directory() -> anyhow::Result<()> {
    let temp = TempDir::new()?;

    fs::write(temp.path().join("file1.txt"), "content1")?;
    fs::write(temp.path().join("file2.txt"), "content2")?;

    let entries = list_directory(temp.path(), ListOptions::default())?;
    assert_eq!(entries.len(), 2);

    Ok(())
}

#[tokio::test]
async fn test_list_directory_pattern() -> anyhow::Result<()> {
    let temp = TempDir::new()?;

    fs::write(temp.path().join("file1.txt"), "content1")?;
    fs::write(temp.path().join("file2.rs"), "content2")?;

    let entries = list_directory(
        temp.path(),
        ListOptions {
            file_pattern: Some("*.txt".into()),
            ..Default::default()
        }
    )?;

    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].file_name().to_string_lossy(),
        "file1.txt"
    );

    Ok(())
}

#[tokio::test]
async fn test_list_directory_recursive() -> anyhow::Result<()> {
    let temp = TempDir::new()?;

    fs::create_dir(temp.path().join("subdir"))?;
    fs::write(temp.path().join("file1.txt"), "content1")?;
    fs::write(temp.path().join("subdir/file2.txt"), "content2")?;

    let entries = list_directory(
        temp.path(),
        ListOptions {
            recursive: true,
            ..Default::default()
        }
    )?;

    assert_eq!(entries.len(), 2);

    Ok(())
}