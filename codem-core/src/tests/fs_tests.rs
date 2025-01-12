use std::fs;
use tempfile::TempDir;
use crate::fs_ops::*;

#[tokio::test]
async fn test_read_file() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content")?;

    let (content, metadata) = read_file(&test_file, ReadOptions::default())?;
    assert_eq!(content, "test content");
    assert_eq!(metadata.size, "test content".len() as u64);

    Ok(())
}

#[tokio::test]
async fn test_read_with_line_count() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "line 1\nline 2\nline 3")?;

    let (content, metadata) = read_file(
        &test_file,
        ReadOptions { count_lines: true }
    )?;
    
    assert_eq!(content, "line 1\nline 2\nline 3");
    assert_eq!(metadata.line_count, Some(3));

    Ok(())
}

#[tokio::test]
async fn test_write_file() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");

    let metadata = write_file(&test_file, "new content")?;
    assert_eq!(metadata.size, "new content".len() as u64);

    let content = fs::read_to_string(&test_file)?;
    assert_eq!(content, "new content");

    Ok(())
}