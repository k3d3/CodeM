use crate::{Client, error::ClientError};
use rstest::rstest;
use tempfile::TempDir;

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