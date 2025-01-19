use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test]
async fn test_full_write() {
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir_all(temp_dir.path().join("session")).unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "original content").unwrap();

    let client = create_test_client(temp_dir.path(), None).await;
    let session_id = client.create_session("test").await.unwrap();

    client.read_file(&session_id, &file_path).await.unwrap();

    let result = client
        .write_file_full(
            &session_id,
            &file_path,
            "new content",
            false
        )
        .await
        .unwrap();

    assert_eq!(result.size, "new content".len());
    assert_eq!(result.line_count, 1);
}