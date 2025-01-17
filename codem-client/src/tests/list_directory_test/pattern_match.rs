use std::fs;
use tempfile::TempDir;
use codem_core::types::ListOptions;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_pattern_match() {
    let test_dir = TempDir::new().unwrap();
    let client = create_test_client(test_dir.path(), None);

    fs::create_dir(test_dir.path().join("dir1")).unwrap();
    fs::create_dir(test_dir.path().join("dir2")).unwrap();
    fs::write(test_dir.path().join("file1.txt"), "content").unwrap();
    fs::write(test_dir.path().join("file2.txt"), "content").unwrap();
    fs::write(test_dir.path().join("other.log"), "content").unwrap();

    let session_id = client.create_session("test").await.unwrap();
    let tree = client.list_directory(
        &session_id, 
        test_dir.path(),
        ListOptions {
            include_size: true,
            include_modified: true,
            include_type: true,
            file_pattern: Some("\\.txt$".to_string()), // Use proper regex pattern
            count_lines: true,
            recursive: false,
         }
    ).await.unwrap();

    assert_eq!(tree.children.len(), 2);
    assert!(
        tree.children.iter()
          .map(|entry| entry.entry.path.to_str().unwrap())
          .all(|path| path.ends_with(".txt"))
    );
}