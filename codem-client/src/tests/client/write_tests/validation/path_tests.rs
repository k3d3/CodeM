use crate::{Client, ClientError};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_path_not_allowed() {
    let temp_dir = TempDir::new().unwrap();
    let disallowed_file = temp_dir.path().parent().unwrap().join("test.txt"); 
    fs::write(&disallowed_file, "test").unwrap();

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

    let result = client
        .write_file(
            &session_id,
            &disallowed_file,
            "new".to_string(),
        )
        .await;

    assert!(result.is_err());
}