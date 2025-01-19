use std::{fs, path::Path};
use tempfile::TempDir;
use codem_core::types::ListOptions;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_list_directory() {
    let test_dir = TempDir::new().unwrap();
    let client = create_test_client(test_dir.path(), None).await;
    let session_id = client.create_session("test").await.unwrap();

    // Create test files/dirs in a subdirectory to avoid session dir
    let work_dir = test_dir.path().join("work");
    fs::create_dir(&work_dir).unwrap();
    
    fs::create_dir(work_dir.join("dir1")).unwrap();
    fs::create_dir(work_dir.join("dir2")).unwrap();
    fs::write(work_dir.join("file1"), "content").unwrap();
    fs::write(work_dir.join("file2"), "content").unwrap();

    // List only the work directory
    let tree = client.list_directory(
        &session_id, 
        Some(Path::new("work")),
        ListOptions { 
            include_size: true,
            include_modified: true,
            file_pattern: None,
            count_lines: true,
            recursive: false,
         }
    ).await.unwrap();

    assert_eq!(tree.children.len(), 4);
}