use super::*;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test]
async fn test_list_directory_with_pattern(test_dir: TempDir) -> Result<(), anyhow::Error> {
    let client = create_test_client(test_dir.path());
    let session_id = client.create_session("test").await?;
    
    let options = ListOptions {
        file_pattern: Some(r"\.txt$".to_string()),
        recursive: true,
        ..Default::default()
    };
    
    let result = client.list_directory(&session_id, test_dir.path(), options).await?;
    
    let all_files = collect_files(&result);
    assert_eq!(all_files.len(), 2);
    assert!(all_files.contains(&"file1.txt".to_string()));
    assert!(all_files.contains(&"subdir1/file2.txt".to_string()));
    assert!(!all_files.contains(&"subdir2/file3.rs".to_string()));
    
    Ok(())
}