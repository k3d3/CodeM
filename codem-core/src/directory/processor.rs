use crate::types::{TreeEntry, ListOptions, FileMetadata};
use crate::fs_ops::is_in_git_dir;
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
    // Skip .git directories
    if is_in_git_dir(relative_path) {
        return Ok(());
    }

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

        node.entry.entry_type = "DIR".to_string();

        if options.recursive {
            let subdir_entry = Box::pin(super::list::list_directory(base_path, entry_path, options)).await?;
            node.children = subdir_entry.children;
            
            // Total up line counts from children
            if options.count_lines {
                let mut total_lines = 0;
                for child in &node.children {
                    if let Some(stats) = &child.entry.stats {
                        if let Some(lines) = stats.line_count {
                            total_lines += lines;
                        }
                    }
                }
                node.entry.stats = Some(FileMetadata {
                    line_count: Some(total_lines),
                    size: None,
                    modified: None,
                });
            }
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
    // Skip files in .git directories
    if is_in_git_dir(relative_path) {
        return Ok(());
    }

    let mut node = TreeEntry::default();
    node.entry.path = relative_path.to_path_buf();
    node.entry.is_dir = false;
    node.entry.symlink = is_symlink;

    node.entry.entry_type = "FILE".to_string();

    if options.include_size || options.include_modified || options.count_lines {
        if let Ok(stats) = get_stats(entry_path, options.count_lines).await {
            if options.include_size {
                node.entry.size = stats.size;
            }
            if options.include_modified {
                node.entry.modified = stats.modified;
            }
            node.entry.stats = Some(stats);
        }
    }

    root.children.push(node);
    Ok(())
}