use crate::{Client, Project};
use crate::config::ClientConfig;
use crate::error::ClientError;
use tempfile::TempDir;

#[tokio::test]
async fn test_run_command() {
    // Create a temp directory that will be cleaned up automatically
    let temp = TempDir::new().unwrap();
    let temp_path = temp.path();

    // Create session directory
    std::fs::create_dir_all(temp_path.join("session")).unwrap();

    // Create a test file
    let test_file = temp_path.join("test.txt");
    std::fs::write(&test_file, "test content").unwrap();

    let test_project = Project::new(temp_path.to_path_buf());

    let config = ClientConfig::new(
        vec![test_project],
        temp_path.join("session").join("session.toml"),
        vec![r"^echo .*".to_string()],  // echo is safe
        vec![r"^rm .*".to_string()]     // rm is risky
    ).unwrap();
    let client = Client::new(config);

    let session_id = client.create_session("test").await.unwrap();

    // Safe command succeeds
    let result = client.run_command(&session_id, "echo hello", Some(temp_path), None).await;
    assert!(result.is_ok(), "Safe command failed: {:?}", result);

    // Safe command succeeds with run_command_risky
    let result = client.run_command_risky(&session_id, "echo hello", Some(temp_path), None).await;
    assert!(result.is_ok(), "Safe command failed with run_command_risky: {:?}", result);
    
    // Risky command fails with run_command
    let result = client.run_command(&session_id, "rm test.txt", Some(temp_path), None).await;
    assert!(matches!(result, Err(ClientError::UnsafeCommand { .. })));

    // Risky command succeeds with run_command_risky
    let result = client.run_command_risky(&session_id, "rm test.txt", Some(temp_path), None).await;
    assert!(result.is_ok(), "Risky command failed with run_command_risky: {:?}", result);

    // No need to clean up - TempDir handles that automatically
}