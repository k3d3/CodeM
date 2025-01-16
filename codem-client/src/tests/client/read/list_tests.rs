use crate::Client;
use rstest::rstest;
use std::fs;
use tempfile::TempDir;

#[rstest]
#[tokio::test]
async fn test_basic_list() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    let config = format!(
        r#"[[projects]]
        base_path = "{}"
        name = "test""#,
        temp_dir.path().display()
    );
    fs::write(&config_path, &config).unwrap();

    let client = Client::new(&config_path).unwrap();
    let _session_id = client.create_session("test").await.unwrap();
}