use std::time::Duration;
use tokio;
use tempfile::TempDir;
use crate::Client;

#[tokio::test]
async fn test_read_file() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");
    std::fs::write(&test_file, "test content")?;

    let client = Client::new(vec![temp.path().to_path_buf()])?;
    let session_id = client.run_on_project("test").await?;

    let content = client.read(&session_id, &test_file).await?;
    assert_eq!(content, "test content");

    Ok(())
}

#[tokio::test]
async fn test_read_timestamp_mismatch() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let test_file = temp.path().join("test.txt");
    std::fs::write(&test_file, "test content")?;

    let client = Client::new(vec![temp.path().to_path_buf()])?;
    let session_id = client.run_on_project("test").await?;

    // First read establishes timestamp
    client.read(&session_id, &test_file).await?;

    // Sleep to ensure timestamp changes
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Modify file external to update timestamp
    std::fs::write(&test_file, "modified content")?;

    let result = client.read(&session_id, &test_file).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_read_disallowed_path() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let client = Client::new(vec![temp.path().to_path_buf()])?;

    let session_id = client.run_on_project("test").await?;

    let test_file = tempfile::NamedTempFile::new()?;
    std::fs::write(&test_file, "test content")?;

    let result = client.read(&session_id, test_file.path()).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_session_paths() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let allowed_path = temp.path().join("allowed");
    std::fs::create_dir(&allowed_path)?;

    let client = Client::new(vec![allowed_path.clone()])?;
    let session_id = client.run_on_project("test").await?;

    let sessions = client.get_sessions().await;
    let session = sessions.iter().find(|s| s.id() == &session_id).unwrap();
    assert_eq!(session.allowed_paths(), &[allowed_path]);

    Ok(())
}