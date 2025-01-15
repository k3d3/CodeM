use crate::Client;
use rstest::rstest;
use tempfile::TempDir;

#[rstest]
#[tokio::test]
async fn test_grep_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let file_path = temp_path.join("test.txt");

    std::fs::write(&file_path, "test content\nsearch this\ntest content").unwrap();

    let client = Client::new(vec![temp_path.to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    let matches = client
        .grep_file(&session_id, &file_path, "test")
        .await
        .unwrap();
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].content, "test content");
    assert_eq!(matches[1].content, "test content");
}