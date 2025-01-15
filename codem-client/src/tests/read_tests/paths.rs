use crate::{ClientError, Client};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_read_outside_path() -> Result<(), ClientError> {
    // Setup
    let temp = TempDir::new().unwrap();
    let allowed_path = temp.path().canonicalize().unwrap();

    // Create config
    let config_path = temp.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        allowed_path.display()
    );
    fs::write(&config_path, &config).map_err(ClientError::from)?;

    let client = Client::new(&config_path).await?;
    let session_id = client.create_session("test").await?;

    let outside_file = temp.path().parent().unwrap().join("outside.txt");
    fs::write(&outside_file, "test content").map_err(ClientError::from)?;

    let result = client.read_file(&session_id, &outside_file).await;
    assert!(result.is_err());

    Ok(())
}