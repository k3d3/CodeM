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

        if !self.is_allowed_path(path) {
            return Err(FileError::PathNotAllowed.into());
        }

        let file_match = codem_core::grep::grep_file(
            path,
            &regex,
            &GrepOptions {
                context_before: 3,
                context_after: 3,
                ..Default::default()
            },
        ).await?;

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
        _session_id: &SessionId,
        root: &Path,
        pattern: &str,
        options: &ListOptions,
    ) -> Result<Vec<FileMatch>> {
        let regex = Regex::new(pattern).map_err(|e| FileError::InvalidPattern { pattern: e.to_string() })?;

        if !self.is_allowed_path(root) {
            return Err(FileError::PathNotAllowed.into());
        }

        let file_matches = codem_core::grep::grep_codebase(
            root,
            &regex,
            GrepOptions {
                context_before: 3,
                context_after: 3,
                file_pattern: options.file_pattern.clone(),
                ..Default::default()
            },
        ).await?;

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
    }
}
