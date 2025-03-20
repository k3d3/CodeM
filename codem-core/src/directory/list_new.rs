use crate::types::{TreeEntry, ListOptions};
use crate::error::DirectoryError;
use super::processor_new::{process_file_entry, process_dir_entry};
use ignore::WalkBuilder;
use regex::Regex;
use std::path::Path;
use tokio::fs;

pub async fn list_directory(
    base_path: &Path,
    path: &Path,
    options: &ListOptions,
) -> Result<TreeEntry, DirectoryError> {
    let mut root = TreeEntry::default();
    root.entry.path = path.strip_prefix(base_path).unwrap_or(path).to_path_buf();
    root.entry.is_dir = true;
    root.entry.entry_type = "DIR".to_string();

    // Set up the regex for file pattern matching if provided
    let file_pattern_regex = if let Some(pattern) = options.file_pattern.as_ref() {
        Some(Regex::new(pattern).map_err(DirectoryError::RegexError)?)
    } else {
        None
    };

    // Set up the walker with proper ignore settings
    let mut walker_builder = WalkBuilder::new(path);
    walker_builder
        .ignore(true)  // Use ignore patterns from .gitignore
        .git_global(true) // Use global gitignore
        .git_ignore(true) // Use .gitignore files
        .follow_links(false) // Never follow symlinks for safety
        .require_git(false); // Don't require being in a git repo

    // Set max depth if not recursive
    if !options.recursive {
        walker_builder.max_depth(Some(1));
    }

    let walker = walker_builder.build();

    // Process all the filesystem entries
    for result in walker {
        match result {
            Ok(entry) => {
                let entry_path = entry.path().to_path_buf();
                
                // Skip the root directory itself to match previous behavior
                if entry_path == path {
                    continue;
                }
                
                let relative_path = entry_path.strip_prefix(base_path).unwrap_or(&entry_path);
                
                // Check if the entry matches the file pattern (if provided)
                let matches = file_pattern_regex.as_ref().map(|regex| {
                    regex.is_match(
                        entry_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                    )
                }).unwrap_or(true);

                // Get file type including symlink info
                let metadata = fs::symlink_metadata(&entry_path).await
                    .map_err(DirectoryError::IoError)?;
                let is_symlink = metadata.file_type().is_symlink();
                
                // Process based on entry type
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    process_dir_entry(
                        base_path, 
                        &entry_path, 
                        relative_path, 
                        is_symlink,
                        matches,
                        options,
                        &mut root
                    ).await?;
                } else if entry.file_type().map_or(false, |ft| ft.is_file()) && matches {
                    process_file_entry(
                        &entry_path,
                        relative_path,
                        is_symlink,
                        options,
                        &mut root
                    ).await?;
                }
            },
            Err(err) => {
                eprintln!("Error walking directory: {}", err);
            }
        }
    }

    Ok(root)
}