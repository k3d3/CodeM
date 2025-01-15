use crate::types::{FileMetadata, ListOptions, TreeEntry};
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt};
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
            if matches || options.recursive {
                let mut node = TreeEntry::default();
                node.entry.path = relative_path.to_path_buf();
                node.entry.is_dir = true;
                node.entry.symlink = is_symlink;

                if options.include_modified {
                    if let Ok(metadata) = fs::metadata(&entry_path).await {
                        node.entry.modified = metadata.modified().ok();
                    }
                }

                if options.include_type {
                    node.entry.entry_type = Some("directory".to_string());
                }

                if options.recursive {
                    let subdir_entry = Box::pin(list_directory(base_path, &entry_path, options)).await?;
                    node.children = subdir_entry.children;
                }

                root.children.push(node);
            }
        } else if file_type.is_file() && matches {
            let mut node = TreeEntry::default();
            node.entry.path = relative_path.to_path_buf();
            node.entry.is_dir = false;
            node.entry.symlink = is_symlink;

            if options.include_type {
                node.entry.entry_type = Some("file".to_string());
            }

            if options.include_size || options.include_modified || options.count_lines {
                if let Ok(stats) = get_stats(&entry_path, options.count_lines).await {
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
        }
    }

    Ok(root)
}

async fn get_stats(path: &Path, count_lines: bool) -> io::Result<FileMetadata> {
    let metadata = fs::metadata(path).await?;
    let modified = metadata.modified().unwrap();
    let size = metadata.len();

    let line_count = if count_lines {
        let file = fs::File::open(path).await?;
        let reader = io::BufReader::new(file);
        let mut lines = 0;
        let mut reader = reader.lines();
        while let Some(_) = reader.next_line().await? {
            lines += 1;
        }
        
        Some(lines)
    } else {
        None
    };

    Ok(FileMetadata {
        modified,
        size,
        line_count,
    })
}