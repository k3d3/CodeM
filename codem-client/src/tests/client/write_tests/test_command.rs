use crate::tests::common::setup_test_session;
use rstest::rstest;
use std::fs;
use pretty_assertions::assert_eq;

#[rstest]
#[tokio::test]
async fn test_successful_test_command() {
    let (client, session_id, temp_dir) = setup_test_session(Some("echo test")).await;
    let test_file = temp_dir.join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    let result = client
        .write_file_full(&session_id, &test_file, "new content", true)
        .await;

    assert!(result.is_ok());
    assert_eq!(fs::read_to_string(&test_file).unwrap(), "new content");
}

#[rstest]
#[tokio::test]
async fn test_failed_test_command() {
    let (client, session_id, temp_dir) = setup_test_session(Some("exit 1")).await;
    let test_file = temp_dir.join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    let result = client
        .write_file_full(&session_id, &test_file, "new content", true)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Test command failed"));
    assert_eq!(fs::read_to_string(&test_file).unwrap(), "new content");
}

#[rstest]
#[tokio::test]
async fn test_no_test_command_configured() {
    let (client, session_id, temp_dir) = setup_test_session(None).await;
    let test_file = temp_dir.join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    let result = client
        .write_file_full(&session_id, &test_file, "new content", true)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Test command not configured"));
    assert_eq!(fs::read_to_string(&test_file).unwrap(), "new content");
}

#[rstest]
#[tokio::test]
async fn test_no_test_requested() {
    let (client, session_id, temp_dir) = setup_test_session(None).await;
    let test_file = temp_dir.join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    let result = client
        .write_file_full(&session_id, &test_file, "new content", false)
        .await;

    assert!(result.is_ok());
    assert_eq!(fs::read_to_string(&test_file).unwrap(), "new content");
}