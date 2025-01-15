use crate::{Client, ClientError};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_invalid_session() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test content").unwrap();

    let config_path = temp_dir.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp_dir.path().display()
    );
    fs::write(&config_path, &config).unwrap();

    let client = Client::new(&config_path).await.unwrap(); 
    let invalid_session = "nonexistent".to_string();

    let result = client
        .write_file(
            &invalid_session,
            &file_path,
            "new".to_string(),
        )
        .await;

    assert!(result.is_err());
}