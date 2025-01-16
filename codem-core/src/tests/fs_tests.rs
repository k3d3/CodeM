use tokio::fs;
use crate::fs_read::{read_file, ReadOptions};
use std::io;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_read_file() -> io::Result<()> {
    let test_file = NamedTempFile::new()?;
    test_file.as_file().sync_all()?;

    fs::write(&test_file, "test content").await?;

    let (content, metadata) = read_file(&test_file, ReadOptions::default()).await?;
    assert_eq!(content, "test content");
    assert_eq!(metadata.size.unwrap(), "test content".len() as u64);

    Ok(())
}

#[tokio::test]
async fn test_read_with_line_count() -> io::Result<()> {
    let test_file = NamedTempFile::new()?;
    test_file.as_file().sync_all()?;

    fs::write(&test_file, "line 1\nline 2\nline 3\n").await?;

    let (content, metadata) = read_file(&test_file, ReadOptions { 
        count_lines: true,
    }).await?;
    
    assert_eq!(content, "line 1\nline 2\nline 3\n");
    assert_eq!(metadata.line_count, Some(3));

    Ok(())
}