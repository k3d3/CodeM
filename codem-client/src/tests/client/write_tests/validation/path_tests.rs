use tempfile::TempDir;
use crate::error::ClientError;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_write_file_outside_project() {
    let temp_dir = TempDir::new().unwrap();
    let client = create_test_client(temp_dir.path(), None).await;
    let session_id = client.create_session("test").await.unwrap();
    let disallowed_file = temp_dir.path().parent().unwrap().join("test.txt");

    let result = client
        .write_file_full(
            &session_id,
            &disallowed_file,
            "new",
            false
        )
        .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::PathOutOfScope { .. }));
}