use crate::types::{TreeEntry, ListOptions};
use super::processor::{process_directory, process_file};
use tokio::fs;
use tokio::io;
use std::path::Path;

pub async fn list_directory(
    base_path: &Path,
    path: &Path,
    options: &ListOptions,
) -> io::Result<TreeEntry> {
    let mut root = TreeEntry::default();
    root.entry.path = path.strip_prefix(base_path).unwrap_or(path).to_path_buf();
    root.entry.is_dir = true;
    root.entry.entry_type = Some("directory".to_string());

    let file_pattern_regex = options.file_pattern.as_ref().map(|pattern| {
        regex::Regex::new(pattern).unwrap()
    });

    let mut dir = fs::read_dir(path).await?;
    while let Some(entry) = dir.next_entry().await? {
        let file_type = entry.file_type().await?;
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(base_path).unwrap();

        let matches = file_pattern_regex.as_ref().map(|regex| {
            regex.is_match(relative_path.to_str().unwrap())
        }).unwrap_or(true);

        let is_symlink = file_type.is_symlink();

        if file_type.is_dir() {
            process_directory(base_path, &entry_path, relative_path, is_symlink, matches, options, &mut root).await?;
        } else if file_type.is_file() && matches {
            process_file(&entry_path, relative_path, is_symlink, options, &mut root).await?;
        }
    }

    Ok(root)
}