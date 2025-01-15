use crate::{error::FileError, session::SessionId, types::file_ops::FileMatch, Client};
use anyhow::Result;
use codem_core::types::{GrepOptions, ListOptions};
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
        let regex = Regex::new(pattern).map_err(|e| FileError::InvalidPattern { pattern: e.to_string() })?;

        if !self.path_allowed(path) {
            return Err(FileError::PathNotAllowed);
        }

        let file_match = codem_core::grep::grep_file(
            path,
            &regex,
            &GrepOptions {
                context_before: 3,
                context_after: 3,
                ..Default::default()
            },
        )?;

        Ok(file_match
            .map(|fm| {
                fm.matches
                    .into_iter()
                    .map(|m| FileMatch {
                        path: fm.path.clone(),
                        line_number: m.line_number,
                        context: m.context,
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default())
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
        let regex = Regex::new(pattern).map_err(|e| FileError::InvalidPattern { pattern: e.to_string() })?;

        let mut matches = Vec::new();

        if !client.is_allowed_path(root) {
            return Err(FileError::PathNotAllowed);
        }

        let file_matches = codem_core::grep::grep_codebase(
            root,
            &regex,
            GrepOptions {
                context_before: 3,
                context_after: 3,
                file_pattern: options.pattern.clone(),
                ..Default::default()
            },
        )?;

        let mut matches = Vec::new();
        for fm in file_matches {
            matches.extend(
                fm.matches
                    .into_iter()
                    .map(|m| FileMatch {
                        path: fm.path.clone(),
                        line_number: m.line_number,
                        context: m.context,
                    }),
            );
        }
        Ok(matches)
    })
}
