use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use codem_core::types::Change;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test]
async fn test_partial_match() {
    let file_content = "line1\nline2\nline3\n";

    let temp_dir = TempDir::new().unwrap();
    fs::create_dir_all(temp_dir.path().join("session")).unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, file_content).unwrap();

    let client = create_test_client(temp_dir.path(), None).await;
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    let changes = vec![Change {
        new_str: "updated\n".to_string(),
        old_str: "line2\n".to_string(),
        allow_multiple_matches: false,
        line_range: None,
    }];

    let result = client
        .write_file_partial(
            &session_id,
            &file_path,
            changes,
            false
        )
        .await
        .unwrap();

    let expected = "line1\nupdated\nline3\n";
    assert_eq!(result.size, expected.len());
    assert_eq!(fs::read_to_string(&file_path).unwrap(), expected);
}