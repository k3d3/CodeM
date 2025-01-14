use crate::{
    error::{FileError, Result},
    types::{file_ops::FileMatch, ListOptions, SessionId},
    Client,
};
use regex::Regex;
use std::{future::Future, path::Path, pin::Pin};

impl Client {
    /// Search for a pattern in a single file
    pub async fn grep_file(
        &self,
        _session_id: &SessionId,
        path: &Path,
        pattern: &str,
    ) -> Result<Vec<FileMatch>> {
        let regex = Regex::new(pattern).map_err(|e| FileError::InvalidPattern(e.to_string()))?;

        if !self.is_path_allowed(path) {
            return Err(FileError::PathNotAllowed);
        }

        let content = std::fs::read_to_string(path)?;
        let mut matches = Vec::new();

        for (line_number, line) in content.lines().enumerate() {
            if regex.is_match(line) {
                matches.push(FileMatch {
                    path: path.to_path_buf(),
                    line_number: line_number + 1,
                    content: line.to_string(),
                });
            }
        }

        Ok(matches)
    }

    /// Search for a pattern across multiple files
    pub async fn grep_codebase(
        &self,
        session_id: &SessionId,
        root: &Path,
        pattern: &str,
        options: &ListOptions,
    ) -> Result<Vec<FileMatch>> {
        _grep_codebase(self, session_id, root, pattern, options).await
    }
}

/// Implementation separated to avoid self-reference issues
fn _grep_codebase<'a>(
    client: &'a Client,
    session_id: &'a SessionId,
    root: &'a Path,
    pattern: &'a str,
    options: &'a ListOptions,
) -> Pin<Box<dyn Future<Output = Result<Vec<FileMatch>>> + 'a>> {
    Box::pin(async move {
        let _regex = Regex::new(pattern).map_err(|e| FileError::InvalidPattern(e.to_string()))?;

        let mut matches = Vec::new();

        if !client.is_path_allowed(root) {
            return Err(FileError::PathNotAllowed);
        }

        let entries = std::fs::read_dir(root)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let is_dir = entry.file_type()?.is_dir();

            if is_dir && options.recursive {
                matches.extend(_grep_codebase(client, session_id, &path, pattern, options).await?);
            } else if !is_dir {
                if let Some(ref pattern) = options.pattern {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_string();
                        if &ext_str != pattern.trim_start_matches('.') {
                            continue;
                        }
                    }
                }

                matches.extend(client.grep_file(session_id, &path, pattern).await?);
            }
        }

        Ok(matches)
    })
}
