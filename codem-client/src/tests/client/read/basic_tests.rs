use crate::{ClientError, Client};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_basic_read() -> Result<(), ClientError> {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").map_err(ClientError::from)?;

    let config_path = temp.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp.path().display(),
    );
    fs::write(&config_path, &config).map_err(ClientError::from)?;

    let client = Client::new(&config_path).await?;
    let session_id = client.create_session("test").await?;

    let content = client.read_file(&session_id, &test_file).await?;
    assert_eq!(content, "test content");

    Ok(())
}