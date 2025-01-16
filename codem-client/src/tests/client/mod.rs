use std::fs;
use tempfile::tempdir;
use crate::tests::common::create_test_client;

#[tokio::test]
async fn test_grep_integration() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("file1.txt"),
        "test line 1\nfound this\ntest line 3"
    ).unwrap();

    let client = create_test_client(dir.path());
    // Session creation needed for initialization though not used in this test
    let _session_id = client.create_session("test").await.unwrap();

    let matches = client
        .grep_file(dir.path().join("file1.txt"), "found")
        .await
        .unwrap();

    assert_eq!(matches.matches.len(), 1);
    assert_eq!(matches.matches[0].context, "found this");
}