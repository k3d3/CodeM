use std::fs;
use tempfile::TempDir;
use rstest::*;

#[fixture]
pub(crate) fn test_dir() -> TempDir {
    let dir = TempDir::new().unwrap();
    
    fs::create_dir(dir.path().join("subdir1")).unwrap();
    fs::create_dir(dir.path().join("subdir2")).unwrap();
    fs::write(dir.path().join("file1.txt"), "content1").unwrap();
    fs::write(dir.path().join("subdir1/file2.txt"), "content2").unwrap();
    fs::write(dir.path().join("subdir2/file3.rs"), "content3").unwrap();
    
    dir
}

mod basic;
mod pattern_match;