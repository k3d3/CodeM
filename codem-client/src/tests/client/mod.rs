use crate::client::Client;
use rstest::*;
use tempfile::tempdir;
use std::fs;

#[rstest]
#[tokio::test]
async fn test_grep_integration() {
    let client = Client::new();
    let dir = tempdir().unwrap();
    
    fs::write(
        dir.path().join("test.txt"),
        "line one\nfind me\nline three"
    ).unwrap();

    let matches = client.grep_file(
        dir.path().join("test.txt"), 
        "find"
    ).await.unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].line, "find me");
    assert_eq!(matches[0].line_number, 2);

    let results = client.grep_codebase(dir.path(), "find").await.unwrap();
    assert_eq!(results.matches.len(), 1);
    assert_eq!(results.files_searched, 1);
    assert_eq!(results.lines_searched, 3);
    assert_eq!(results.pattern, "find");
}