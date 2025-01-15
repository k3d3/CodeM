use rstest::rstest;
use tempfile::TempDir;
use std::fs;

use crate::Client;

#[rstest]
#[tokio::test]
async fn test_full_write() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "original content").unwrap();

    let config_path = temp_dir.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp_dir.path().display()
    );
    fs::write(&config_path, &config).unwrap();

    let client = Client::new(&config_path).await.unwrap();
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    let result = client
        .write_file(
            &session_id,
            &file_path,
            "new content".to_string(),
        )
        .await
        .unwrap();

    assert_eq!(result.size, "new content".len());
    assert_eq!(result.line_count, 1);
}