use crate::{
    Client,
    session::SessionManager
};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_invalid_session() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test").unwrap();

    let session_manager = SessionManager::new_test();
    let client = Client::new(session_manager);

    let result = client
        .write_file(
            "invalid",
            &file_path,
            "new",
        )
        .await;

    assert!(result.is_err());
}