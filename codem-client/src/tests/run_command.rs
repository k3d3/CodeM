use rstest::rstest;
use tempfile::TempDir;
use crate::error::ClientError;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test] 
async fn test_run_command() {
    // Create temporary directory for test
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let client = create_test_client(temp_path, None);
    let session_id = client.create_session("test").await.unwrap();
    
    // Test safe command
    let result = client.run_command(&session_id, "echo hello_world", None, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.trim(), "hello_world");

    // Test invalid safe command (has space)
    let result = client.run_command(&session_id, "echo hello world", None, None).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::UnsafeCommand { .. }));

    // Test risky command without timeout
    let result = client.run_command(&session_id, "rm test.txt", None, None).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::UnsafeCommand { .. }));

    // Test risky command with timeout - should still be rejected
    let result = client.run_command(&session_id, "rm test.txt", None, Some(30)).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::UnsafeCommand { .. }));
}