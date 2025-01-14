use crate::{types::ListOptions, Client};
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

#[rstest]
#[tokio::test]
async fn test_list_directory_with_options() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    std::fs::write(temp_path.join("test1.txt"), "content1").unwrap();
    std::fs::write(temp_path.join("test2.doc"), "content2").unwrap();
    std::fs::create_dir(temp_path.join("subdir")).unwrap();

    let client = Client::new(vec![temp_path.to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    let options = ListOptions {
        recursive: false,
        include_stats: true,
        pattern: Some("*.txt".to_string()),
    };

    let entries = client
        .list_directory(&session_id, temp_path, &options)
        .await
        .unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path.file_name().unwrap(), "test1.txt");
    assert!(entries[0].stats.is_some());
}
