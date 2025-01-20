use crate::types::{TreeEntry, ListOptions};
use crate::error::DirectoryError;
use super::processor::{process_directory, process_file};
use tokio::fs;
use std::path::Path;

pub async fn list_directory(
    base_path: &Path,
    path: &Path,
    options: &ListOptions,
) -> Result<TreeEntry, DirectoryError> {
    let mut root = TreeEntry::default();
    root.entry.path = path.strip_prefix(base_path).unwrap_or(path).to_path_buf();
    root.entry.is_dir = true;
    root.entry.entry_type = "DIR".to_string();

    let file_pattern_regex = if let Some(pattern) = options.file_pattern.as_ref() {
        Some(regex::Regex::new(pattern).map_err(DirectoryError::RegexError)?)
    } else {
        None
    };

    let mut dir = fs::read_dir(path).await.map_err(DirectoryError::IoError)?;
    while let Some(entry) = dir.next_entry().await.map_err(DirectoryError::IoError)? {
        let file_type = entry.file_type().await.map_err(DirectoryError::IoError)?;
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(base_path).unwrap();

        let matches = file_pattern_regex.as_ref().map(|regex| {
            regex.is_match(
                entry_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
            )
        }).unwrap_or(true);

        let is_symlink = file_type.is_symlink();

        if file_type.is_dir() {
            process_directory(base_path, &entry_path, relative_path, is_symlink, matches, options, &mut root).await?
        } else if file_type.is_file() && matches {
            process_file(&entry_path, relative_path, is_symlink, options, &mut root).await?;
        }
    }

    Ok(root)
}