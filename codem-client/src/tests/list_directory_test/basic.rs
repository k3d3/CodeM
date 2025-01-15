use super::*;

#[rstest]
#[tokio::test]
async fn test_list_directory_basic(test_dir: TempDir) -> io::Result<()> {
    let client = Client::new();
    let options = ListOptions::default();
    
    let result = client.list_directory(test_dir.path(), options).await?;
    
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