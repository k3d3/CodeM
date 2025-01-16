use std::fs;
use tempfile::TempDir;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_invalid_session() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt"); 
    fs::write(&file_path, "test").unwrap();

    let client = create_test_client(temp_dir.path());

    let result = client
        .write_file_full(
            "invalid-session",
            &file_path,
            "new",
        )
        .await;

    assert!(result.is_err());
}