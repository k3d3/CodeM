pub mod write_tests;
pub mod read;
pub mod read_tests;

use tempfile::TempDir;
use std::fs;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_create_and_get_session() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("session")).unwrap();
    let client = create_test_client(dir.path(), None);
    let session_id = client.create_session("test").await.unwrap();

    assert!(session_id.len() > 0);
}