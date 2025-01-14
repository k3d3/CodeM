use crate::types::FileMetadata;
use crate::types::{ListOptions, ListEntry};
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt};
use std::path::Path;

pub async fn list_directory(
    base_path: &Path,
    path: &Path,
    options: &ListOptions,
) -> io::Result<Vec<ListEntry>> {
    let mut entries = Vec::new();

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
        let symlink = if is_symlink {
            Some(fs::read_link(&entry_path).await?)
        } else {
            None
        };

        if file_type.is_dir() {
            if matches {
                entries.push(ListEntry {
                    path: relative_path.to_path_buf(),
                    is_dir: true,
                    symlink,
                    stats: None,
                });
            }
            if options.recursive && is_symlink {
                entries.extend(Box::pin(list_directory(entry_path.as_path(), relative_path, options)).await?);
            }
        } else if file_type.is_file() {
            if matches {
                let stats = Some(get_stats(path, options.count_lines).await?);
                entries.push(ListEntry {
                    path: relative_path.to_path_buf(),
                    is_dir: false,
                    symlink,
                    stats,
                });
            }
        }
    }

    Ok(entries)
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