use crate::types::{TreeEntry, ListOptions};
use super::stats::get_stats;
use tokio::fs;
use tokio::io;
use std::path::Path;

pub async fn process_directory(
    base_path: &Path,
    entry_path: &Path,
    relative_path: &Path,
    is_symlink: bool,
    matches: bool,
    options: &ListOptions,
    root: &mut TreeEntry,
) -> io::Result<()> {
    if matches || options.recursive {
        let mut node = TreeEntry::default();
        node.entry.path = relative_path.to_path_buf();
        node.entry.is_dir = true;
        node.entry.symlink = is_symlink;

        if options.include_modified {
            if let Ok(metadata) = fs::metadata(entry_path).await {
                node.entry.modified = metadata.modified().ok();
            }
        }

        if options.include_type {
            node.entry.entry_type = Some("directory".to_string());
        }

        if options.recursive {
            let subdir_entry = Box::pin(super::list::list_directory(base_path, entry_path, options)).await?;
            node.children = subdir_entry.children;
        }

        root.children.push(node);
    }
    Ok(())
}

pub async fn process_file(
    entry_path: &Path,
    relative_path: &Path,
    is_symlink: bool,
    options: &ListOptions,
    root: &mut TreeEntry,
) -> io::Result<()> {
    let mut node = TreeEntry::default();
    node.entry.path = relative_path.to_path_buf();
    node.entry.is_dir = false;
    node.entry.symlink = is_symlink;

    if options.include_type {
        node.entry.entry_type = Some("file".to_string());
    }

    if options.include_size || options.include_modified || options.count_lines {
        if let Ok(stats) = get_stats(entry_path, options.count_lines).await {
            if options.include_size {
                node.entry.size = Some(stats.size);
            }
            if options.include_modified {
                node.entry.modified = Some(stats.modified);
            }
            node.entry.stats = Some(stats);
        }
    }

    root.children.push(node);
    Ok(())
}