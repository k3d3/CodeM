use std::{path::PathBuf, time::Duration};
use tokio;
use tempfile::TempDir;
use crate::{Client, error::ClientError};
use rstest::rstest;
use serde_json::json;

#[rstest]
#[tokio::test]
async fn test_read_file() -> Result<(), ClientError> {
    let temp = TempDir::new().map_err(ClientError::from)?;
    let test_file = temp.path().join("test.txt");
    std::fs::write(&test_file, "test content").map_err(ClientError::from)?;

    // Create test config
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

    let content = client.read(&session_id, &test_file).await?;
    assert_eq!(content, "test content");

    Ok(())
}

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

#[rstest]
#[tokio::test]
async fn test_read_disallowed_path() -> Result<(), ClientError> {
    let temp = TempDir::new().map_err(ClientError::from)?;
    let allowed_dir = temp.path().join("allowed");
    std::fs::create_dir(&allowed_dir).map_err(ClientError::from)?;

    let config_path = temp.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        allowed_dir.display()
    );
    std::fs::write(&config_path, config).map_err(ClientError::from)?;

    let client = Client::new(&config_path).await?;
    let session_id = client.create_session("test").await?;

    let outside_file = temp.path().join("outside.txt");
    std::fs::write(&outside_file, "test content").map_err(ClientError::from)?;

    let result = client.read(&session_id, &outside_file).await;
    assert!(result.is_err());

    Ok(())
}

#[rstest]
#[tokio::test] 
async fn test_session_paths() -> Result<(), ClientError> {
    let temp = TempDir::new().map_err(ClientError::from)?;
    let allowed_path = temp.path().join("allowed");
    std::fs::create_dir(&allowed_path).map_err(ClientError::from)?;

    let config_path = temp.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        allowed_path.display()
    );
    std::fs::write(&config_path, config).map_err(ClientError::from)?;

    let client = Client::new(&config_path).await?;
    let session_id = client.create_session("test").await?;

    let sessions = client.get_sessions().await;
    let session = sessions.iter().find(|s| s.id() == &session_id).unwrap();
    assert_eq!(session.allowed_paths(), &[allowed_path]);

    Ok(())
}