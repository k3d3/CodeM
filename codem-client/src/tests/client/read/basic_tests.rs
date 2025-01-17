use crate::{ClientError, Client, Project, config::ClientConfig};
use std::fs;
use tempfile::TempDir;

fn create_test_config(temp_dir: &TempDir) -> ClientConfig {
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    let projects = vec![project];

    let session_dir = temp_dir.path().join("session");
    fs::create_dir(&session_dir).unwrap();

    ClientConfig::new(
        projects,
        session_dir.join("session.toml"),
        vec![], 
        vec![]
    ).unwrap()
}

#[tokio::test]
async fn test_basic_read() -> Result<(), ClientError> {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").map_err(ClientError::from)?;

    let config = create_test_config(&temp);
    let client = Client::new(config);
    let session_id = client.create_session("test").await?;

    let content = client.read_file(&session_id, &test_file).await?;
    assert_eq!(content, "test content");

    Ok(())
}