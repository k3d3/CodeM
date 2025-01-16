use std::fs;
use tempfile::TempDir;
use codem_core::types::ListOptions;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_list_directory() {
    let test_dir = TempDir::new().unwrap();
    let client = create_test_client(test_dir.path(), None);

    fs::create_dir(test_dir.path().join("dir1")).unwrap();
    fs::create_dir(test_dir.path().join("dir2")).unwrap();
    fs::write(test_dir.path().join("file1"), "content").unwrap();
    fs::write(test_dir.path().join("file2"), "content").unwrap();

    let session_id = client.create_session("test").await.unwrap();
    let tree = client.list_directory(
        &session_id, 
        test_dir.path(),
        ListOptions { 
            include_size: true,
            include_modified: true,
            include_type: true,
            file_pattern: None,
            count_lines: true,
         }
    ).await.unwrap();

    assert_eq!(tree.children.len(), 4);
}