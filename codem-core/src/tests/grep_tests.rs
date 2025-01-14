use crate::grep::{grep_codebase, grep_file};
use crate::types::GrepOptions;
use regex::Regex;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_grep_file() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let file = temp.path().join("test.txt");
    fs::write(&file, "line one\nline two\nline three")?;

    let pattern = Regex::new("line").unwrap();
    let matches = grep_file(&file, &pattern, &GrepOptions::default())?;

    assert_eq!(matches.len(), 3);
    assert_eq!(matches[0].line_number, 1);
    assert_eq!(matches[1].line_number, 2);
    assert_eq!(matches[2].line_number, 3);

    Ok(())
}

#[tokio::test]
async fn test_grep_codebase() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    fs::create_dir(temp.path().join("subdir"))?;

    fs::write(temp.path().join("test1.txt"), "line one\nline two")?;
    fs::write(temp.path().join("subdir/test2.txt"), "line three")?;
    fs::write(temp.path().join("test.rs"), "other content")?;

    let pattern = Regex::new("line").unwrap();
    let matches = grep_codebase(
        temp.path(),
        &pattern,
        GrepOptions {
            file_pattern: Some("*.txt".into()),
            ..Default::default()
        },
    )?;

    assert_eq!(matches.len(), 3);

    Ok(())
}
