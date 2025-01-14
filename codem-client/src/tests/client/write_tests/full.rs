use rstest::rstest;
use tempfile::TempDir;

use crate::types::{CheckOptions, WriteOperation};
use crate::Client;

#[rstest]
#[tokio::test]
async fn test_write_full() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let file_path = temp_path.join("test.txt");

    let client = Client::new(vec![temp_path.to_path_buf()]).unwrap();
    let session_id = client.create_session("test").await.unwrap();

    let content = "new content\nline 2";
    let result = client
        .write(
            &session_id,
            &file_path,
            WriteOperation::Full(content.to_string()),
            CheckOptions::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.matches.len(), 1);
    assert_eq!(result.matches[0].content, content);

    let file_content = tokio::fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(file_content, content);
}
