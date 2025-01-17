use crate::{Client, Project, config::ClientConfig};
use rstest::rstest;
use tempfile::TempDir;

fn create_test_config(temp_dir: &TempDir) -> ClientConfig {
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    let projects = vec![project];

    // Create session directory
    let session_dir = temp_dir.path().join("session");
    std::fs::create_dir(&session_dir).unwrap();

    ClientConfig::new(
        projects,
        session_dir.join("session.toml"),
        vec![], 
        vec![]
    ).unwrap()
}

#[rstest]
#[tokio::test]
async fn test_basic_list() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    
    let client = Client::new(config);
    let _session_id = client.create_session("test").await.unwrap();
}