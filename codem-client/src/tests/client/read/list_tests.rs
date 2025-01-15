use crate::{types::ListOptions, Client};
use rstest::rstest;
use tempfile::TempDir;

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