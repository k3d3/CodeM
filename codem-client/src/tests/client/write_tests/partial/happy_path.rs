use rstest::rstest;
use tempfile::TempDir;
use std::fs;
use std::sync::Arc;
use std::collections::HashMap;
use codem_core::types::Change;

use crate::{Client, project::Project, session::SessionManager};

#[rstest]
#[tokio::test]
async fn test_partial_happy_path() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut projects = HashMap::new();
    let mut project = Project::new(temp_dir.path().to_path_buf());
    project.allowed_paths = Some(vec![temp_dir.path().to_path_buf()]);
    projects.insert("test".to_string(), Arc::new(project));
    
    let sessions = SessionManager::new(projects, None);
    let client = Client::new(sessions);
    let session_id = client.create_session("test").await.unwrap();

    // Read first to cache timestamp
    client.read_file(&session_id, &file_path).await.unwrap();

    let changes = vec![Change {
        new_str: "updated_line\n".to_string(),
        old_str: "line2\n".to_string(),
        allow_multiple_matches: false,
    }];

    let result = client
        .write_file_partial(
            &session_id,
            &file_path,
            changes
        )
        .await
        .unwrap();

    let expected_content = "line1\nupdated_line\nline3\n";
    assert_eq!(result.size, expected_content.len());
    assert_eq!(result.line_count, 3);

    let actual_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(actual_content, expected_content);
}