use crate::{Client, ClientError};
use codem_core::types::Change;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_pattern_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "content").unwrap();

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

    let _ = client.read_file(&session_id, &file_path).await.unwrap();

    let changes = vec![Change {
        old_str: "nonexistent".to_string(),
        new_str: "new".to_string(),
        allow_multiple_matches: false,
    }];

    let result = client
        .write_file_partial(
            &session_id,
            &file_path,
            changes,
        )
        .await;

    assert!(result.is_err());
}