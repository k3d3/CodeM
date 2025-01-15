use std::time::Duration;
use crate::{Client, error::ClientError};
use rstest::rstest;
use tempfile::TempDir;
use tokio;

#[rstest]
#[tokio::test]
async fn test_read_timestamp_mismatch() -> Result<(), ClientError> {
    let temp = TempDir::new().map_err(ClientError::from)?;
    let test_file = temp.path().join("test.txt");
    std::fs::write(&test_file, "test content").map_err(ClientError::from)?;

    let config_path = temp.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        test_file.parent().unwrap().display()
    );
    std::fs::write(&config_path, config).map_err(ClientError::from)?;

    let client = Client::new(&config_path).await?;
    let session_id = client.create_session("test").await?;

    // First read establishes timestamp
    client.read(&session_id, &test_file).await?;

    // Sleep to ensure timestamp changes
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Modify file external to update timestamp
    std::fs::write(&test_file, "modified content").map_err(ClientError::from)?;

    let result = client.read(&session_id, &test_file).await;
    assert!(result.is_err());

    Ok(())
}