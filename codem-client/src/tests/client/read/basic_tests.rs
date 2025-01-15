use crate::Client;
use rstest::rstest;
use tempfile::TempDir;

#[rstest]
#[tokio::test]
async fn test_read_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let file_path = temp_path.join("test.txt");

    std::fs::write(&file_path, "test content\nline 2\nline 3").unwrap();

    let client = Client::new(vec![temp_path.to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    let result = client.read(&session_id, &file_path).await.unwrap();
    assert_eq!(result.content, "test content\nline 2\nline 3");
}