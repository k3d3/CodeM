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
    let file_match = grep_file(&file, &pattern, &GrepOptions::default())?.unwrap();

    assert_eq!(file_match.matches.len(), 3);
    assert_eq!(file_match.path, file);
    assert_eq!(file_match.matches[0].line_number, 1);
    assert_eq!(file_match.matches[1].line_number, 2);
    assert_eq!(file_match.matches[2].line_number, 3);
    
    // Check context contains the matched line
    assert!(file_match.matches[0].context.contains("line one"));
    assert!(file_match.matches[1].context.contains("line two"));
    assert!(file_match.matches[2].context.contains("line three"));

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
    let file_matches = grep_codebase(
        temp.path(),
        &pattern,
        GrepOptions {
            file_pattern: Some("*.txt".into()),
            ..Default::default()
        },
    )?;

    for m in &file_matches {
        println!("Found match in: {:?}", m.path);
    }

    assert_eq!(file_matches.len(), 2); // Should find 2 files
    
    
    // Check first file (test1.txt)
    assert!(file_matches[0].path.ends_with("test1.txt"));
    assert_eq!(file_matches[0].matches.len(), 2); // two matches in first file
    
    // Check second file (test2.txt)
    assert!(file_matches[1].path.ends_with("test2.txt"));
    assert_eq!(file_matches[1].matches.len(), 1); // one match in second file

    Ok(())
}