use tempfile::TempDir;
use std::fs;
use tokio::io::AsyncWriteExt;

use crate::directory::list_directory;
use crate::types::ListOptions;
use std::path::PathBuf;

#[tokio::test]
async fn test_git_directory_ignored_in_listing() {
    let temp = TempDir::new().unwrap();
    let git_dir = temp.path().join(".git");
    let git_file = git_dir.join("HEAD");
    let normal_dir = temp.path().join("src");
    let normal_file = normal_dir.join("main.rs");

    // Create test directory structure
    fs::create_dir_all(&git_dir).unwrap();
    fs::create_dir_all(&normal_dir).unwrap();

    // Create test files
    let mut file = tokio::fs::File::create(&git_file).await.unwrap();
    file.write_all(b"test git content").await.unwrap();

    let mut file = tokio::fs::File::create(&normal_file).await.unwrap();
    file.write_all(b"test normal content").await.unwrap();

    // Test directory listing
    let options = ListOptions {
        recursive: true,
        include_size: true,
        include_modified: true,
        count_lines: true,
        ..Default::default()
    };

    let entries = list_directory(temp.path(), temp.path(), &options)
        .await
        .unwrap();

    // Helper function to collect all paths
    fn collect_paths(entries: &crate::types::TreeEntry, paths: &mut Vec<PathBuf>) {
        paths.push(entries.entry.path.clone());
        for child in &entries.children {
            collect_paths(child, paths);
        }
    }

    let mut found_paths = Vec::new();
    collect_paths(&entries, &mut found_paths);

    // Check that .git directory and its contents are not included
    for path in &found_paths {
        assert!(!path.to_str().unwrap().contains(".git"), 
            "Found unexpected git path: {:?}", path);
    }

    // Check that normal files are included
    let normal_path = PathBuf::from("src/main.rs");
    assert!(found_paths.contains(&normal_path), 
        "Did not find expected path: {:?}", normal_path);
}