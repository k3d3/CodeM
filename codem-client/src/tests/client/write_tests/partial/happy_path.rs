use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use codem_core::types::Change;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test]
async fn test_partial_happy_path() {
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir_all(temp_dir.path().join("session")).unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let client = create_test_client(temp_dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

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
            changes,
            false
        )
        .await
        .unwrap();

    let expected_content = "line1\nupdated_line\nline3\n";
    assert_eq!(result.size, expected_content.len());
    assert_eq!(result.line_count, 3);

    let actual_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(actual_content, expected_content);
}