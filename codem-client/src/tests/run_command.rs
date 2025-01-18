use crate::{Client, Project};
use crate::config::ClientConfig;
use crate::error::ClientError;
use tempfile::TempDir;

#[tokio::test]
async fn test_run_command() {
    // Create a temp directory that will be cleaned up automatically
    let temp = TempDir::new().unwrap();
    let temp_path = temp.path();

    // Create session directories
    std::fs::create_dir_all(temp_path.join("session")).unwrap();
    std::fs::create_dir_all(temp_path.join("session2")).unwrap();

    // Create a test file
    let test_file = temp_path.join("test.txt");
    std::fs::write(&test_file, "test content").unwrap();

    let mut test_project = Project::new(temp_path.to_path_buf());
    test_project.test_command = Some("echo test output".to_string());

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

    // Test command returns output
    let result = client.run_test_command(&session_id).await;
    assert!(result.is_ok(), "Test command failed: {:?}", result);
    assert_eq!(result.unwrap().trim(), "test output");

    // No test command configured returns error
    let mut test_project2 = Project::new(temp_path.to_path_buf());
    test_project2.test_command = None;

    let config2 = ClientConfig::new(
        vec![test_project2],
        temp_path.join("session2").join("session.toml"),
        vec![r"^echo .*".to_string()],  // echo is safe
        vec![r"^rm .*".to_string()]     // rm is risky
    ).unwrap();
    let client2 = Client::new(config2);

    let session_id2 = client2.create_session("test").await.unwrap();
    let result = client2.run_test_command(&session_id2).await;
    assert!(matches!(result, Err(ClientError::TestCommandNotConfigured)));

    // No need to clean up - TempDir handles that automatically
}