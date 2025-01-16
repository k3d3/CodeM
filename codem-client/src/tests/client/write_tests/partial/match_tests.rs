use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use codem_core::types::Change;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test]
async fn test_multiple_matches() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "line1\nline2\nline2\n").unwrap();

    let client = create_test_client(temp_dir.path());
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    let changes = vec![Change {
        new_str: "updated_line\n".to_string(),
        old_str: "line2\n".to_string(),
        allow_multiple_matches: false,
    }];

    let result = client
        .write_file_partial(
            &session_id,
            &file_path,
            changes
        )
        .await;

    assert!(result.is_err());
}