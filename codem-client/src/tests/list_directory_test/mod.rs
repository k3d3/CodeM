use crate::client::Client;
use codem_core::types::{ListOptions, TreeEntry};
use std::fs;
use tempfile::TempDir;
use tokio::io;
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

pub(crate) fn collect_files(entry: &TreeEntry) -> Vec<String> {
    let mut files = Vec::new();
    if !entry.entry.is_dir {
        files.push(entry.entry.path.to_str().unwrap().to_string());
    }
    for child in &entry.children {
        files.extend(collect_files(child));
    }
    files
}

mod basic;
mod pattern_match;