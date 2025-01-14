use crate::{
    fs_ops::*,
    types::{PartialWrite, PartialWriteInner, WriteOperation},
};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_read_file() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content")?;

    let (content, metadata) = read_file(&test_file, ReadOptions::default()).await?;
    assert_eq!(content, "test content");
    assert_eq!(metadata.size, "test content".len() as u64);

    Ok(())
}

#[tokio::test]
async fn test_read_with_line_count() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "line 1\nline 2\nline 3")?;

    let (content, metadata) = read_file(&test_file, ReadOptions { count_lines: true }).await?;

    assert_eq!(content, "line 1\nline 2\nline 3");
    assert_eq!(metadata.line_count, Some(3));

    Ok(())
}

#[tokio::test]
async fn test_write_file_full() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");

    let write_op = WriteOperation::Full("new content".to_string());
    let metadata = write_file(&test_file, write_op, None).await.unwrap();
    assert_eq!(metadata.size, "new content".len());

    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "new content");
}

#[tokio::test]
async fn test_write_partial() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");

    // generate string with 20 lines
    let mut content = String::new();
    let mut expected = String::new();
    for i in 0..20 {
        content.push_str(&format!("line {}\n", i*2+1));
        expected.push_str(&format!("BOOP\nLINE {}\n", i*2+1));
    }

    // write the file regularly first
    fs::write(&test_file, content).unwrap();

    let write_op = WriteOperation::Partial(PartialWrite {
        context_lines: 1,
        return_full_content: true,
        writes: vec![PartialWriteInner {
            old_str: "line".to_string(),
            new_str: "BOOP\nLINE".to_string(),
            allow_multiple_matches: false,
        }],
    });

    let metadata = write_file(&test_file, write_op, None).await.unwrap();
    assert_eq!(metadata.size, expected.len());

    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, expected);

    // partial write context
    let expected_partial_write = "line 6\nnew line 7\nand a line after\nline 8";
    assert_eq!(metadata.partial_write_result.unwrap().content[0].context, expected_partial_write);

}
