use std::fs;
use tempfile::TempDir;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_path_not_allowed() {
    let temp_dir = TempDir::new().unwrap();
    let disallowed_file = temp_dir.path().parent().unwrap().join("test.txt"); 
    fs::write(&disallowed_file, "test").unwrap();

    let client = create_test_client(temp_dir.path());
    let session_id = client.create_session("test").await.unwrap();

    let result = client
        .write_file_full(
            &session_id,
            &disallowed_file,
            "new",
        )
        .await;

    assert!(result.is_err());
}