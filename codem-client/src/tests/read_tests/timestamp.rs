use crate::{Client, ClientError};
use std::{fs, time::Duration};
use tempfile::TempDir;

#[tokio::test]
async fn test_file_changed() -> Result<(), ClientError> {
    // Setup
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "original content").map_err(ClientError::from)?;

    // Create config
    let config_path = temp.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp.path().display()
    );
    fs::write(&config_path, &config).map_err(ClientError::from)?;

    let client = Client::new(&config_path).await?;
    let session_id = client.create_session("test").await?;

    // First read establishes timestamp
    let _ = client.read_file(&session_id, &test_file).await?;

    // Sleep to ensure timestamp changes
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Modify file externally
    fs::write(&test_file, "modified content").map_err(ClientError::from)?;

    let result = client.read_file(&session_id, &test_file).await;
    assert!(result.is_err());

    Ok(())
}