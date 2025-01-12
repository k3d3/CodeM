use crate::{
    error::{FileError, Result},
    types::{ListOptions, ListEntry, SessionId},
    Client,
};
use codem_core::types::FileMetadata;
use std::path::Path;

impl Client {
    /// List contents of a directory with optional filtering and stats 
    pub async fn list_directory(
        &self,
        _session_id: &SessionId,
        path: &Path,
        options: &ListOptions,
    ) -> Result<Vec<ListEntry>> {
        if !self.is_path_allowed(path) {
            return Err(FileError::PathNotAllowed);
        }

        let mut entries = Vec::new();
        let read_dir = std::fs::read_dir(path)?;

        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let is_dir = metadata.is_dir();

            // Skip directories if recursive is false
            if is_dir && !options.recursive {
                continue;
            }

            // Apply file pattern filter if provided
            if !is_dir {
                if let Some(ref pattern) = options.pattern {
                    // Convert pattern from glob-like syntax to file extension
                    let ext = pattern.trim_start_matches('*').trim_start_matches('.');
                    if let Some(file_ext) = path.extension() {
                        if file_ext != ext {
                            continue;
                        }
                    }
                }
            }

            entries.push(ListEntry {
                path,
                is_dir,
                stats: if options.include_stats {
                    Some(FileMetadata {
                        modified: metadata.modified()?,
                        size: metadata.len(),
                        line_count: None,
                    })
                } else {
                    None
                },
            });
        }

        // Sort entries by path
        entries.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));

        Ok(entries)
    }
}