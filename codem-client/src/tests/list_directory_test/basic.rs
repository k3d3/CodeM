use super::*;
use crate::tests::common::create_test_client;

#[rstest]
#[tokio::test]
async fn test_list_directory_basic(test_dir: TempDir) -> Result<(), anyhow::Error> {
    let client = create_test_client(test_dir.path());
    let session_id = client.create_session("test").await?;
    let options = ListOptions::default();
    
    let result = client.list_directory(&session_id, test_dir.path(), options).await?;
    
    assert!(result.entry.is_dir);
    assert_eq!(result.children.len(), 3); // subdir1, subdir2, file1.txt
    
    let file_names: Vec<String> = result.children.iter()
        .map(|entry| entry.entry.path.to_str().unwrap().to_string())
        .collect();
        
    assert!(file_names.contains(&"file1.txt".to_string()));
    assert!(file_names.contains(&"subdir1".to_string()));
    assert!(file_names.contains(&"subdir2".to_string()));
    
    Ok(())
}