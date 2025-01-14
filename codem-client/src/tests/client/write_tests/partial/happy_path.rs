use crate::{
    types::{CheckOptions, WriteOperation},
    Client,
};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_partial_write() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "original content").unwrap();

    let client = Client::new(vec![temp_dir.path().to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    let _ = client.read(&session_id, &file_path).await.unwrap();

    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Partial {
                old_str: "original".to_string(),
                new_str: "modified".to_string(),
            },
            CheckOptions::default(),
        )
        .await
        .unwrap();

    assert!(result.matches.len() >= 1);
    assert!(fs::read_to_string(&file_path)
        .unwrap()
        .contains("modified content"));
    assert!(result.original_content.is_some());
}
