use crate::fs_ops::*;
use std::fs;
use tempfile::TempDir;
use tokio_test::block_on;

#[test]
fn test_read_file() -> anyhow::Result<()> {
    block_on(async {
        let temp = TempDir::new()?;
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "test content")?;

        let (content, metadata) = read_file(&test_file, ReadOptions::default()).await?;
        assert_eq!(content, "test content");
        assert_eq!(metadata.size, "test content".len() as u64);

        Ok(())
    })
}

#[test]
fn test_read_with_line_count() -> anyhow::Result<()> {
    block_on(async {
        let temp = TempDir::new()?;
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "line 1\nline 2\nline 3")?;

        let (content, metadata) = read_file(&test_file, ReadOptions { count_lines: true }).await?;

        assert_eq!(content, "line 1\nline 2\nline 3");
        assert_eq!(metadata.line_count, Some(3));

        Ok(())
    })
}
