use crate::types::{TreeEntry, ListOptions};
use crate::error::DirectoryError;
use super::stats::get_stats;
use tokio::fs;
use std::path::Path;

pub async fn process_dir_entry(
    _base_path: &Path,
    entry_path: &Path,
    relative_path: &Path,
    is_symlink: bool,
    matches: bool,
    options: &ListOptions,
    root: &mut TreeEntry,
) -> Result<(), DirectoryError> {
    if matches || options.recursive {
        let mut node = TreeEntry::default();
        node.entry.path = relative_path.to_path_buf();
        node.entry.is_dir = true;
        node.entry.symlink = is_symlink;

        if options.include_modified {
            let metadata = fs::metadata(entry_path).await.map_err(DirectoryError::IoError)?;
            node.entry.modified = metadata.modified().ok();
        }

        node.entry.entry_type = "DIR".to_string();
        root.children.push(node);
    }
    Ok(())
}

pub async fn process_file_entry(
    entry_path: &Path,
    relative_path: &Path,
    is_symlink: bool,
    options: &ListOptions,
    root: &mut TreeEntry,
) -> Result<(), DirectoryError> {
    let mut node = TreeEntry::default();
    node.entry.path = relative_path.to_path_buf();
    node.entry.is_dir = false;
    node.entry.symlink = is_symlink;
    node.entry.entry_type = "FILE".to_string();

    if options.include_size || options.include_modified || options.count_lines {
        let stats = get_stats(entry_path, options.count_lines).await.map_err(DirectoryError::IoError)?;
        if options.include_size {
            node.entry.size = stats.size;
        }
        if options.include_modified {
            node.entry.modified = stats.modified;
        }
        node.entry.stats = Some(stats);
    }

    root.children.push(node);
    Ok(())
}