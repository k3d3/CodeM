use std::path::Path;
use regex::Regex;
use crate::{
    error::{ClientError, OperationError},
    session::SessionId,
    types::file_ops::GrepMatch,
};
use codem_core::{
    grep::grep_file,
    types::GrepOptions
};

pub(super) async fn grep_matches(
    session_id: &SessionId,
    path: &Path,
    pattern: &str,
) -> Result<Vec<GrepMatch>, ClientError> {
    let pattern_regex = Regex::new(pattern)
        .map_err(|e| OperationError::PatternError { 
            message: e.to_string() 
        })?;

    let options = GrepOptions::default();
    
    let result = grep_file(path, &pattern_regex, &options)
        .await
        .map_err(|e| OperationError::PatternError { 
            message: e.to_string() 
        })?
        .unwrap_or_default();

    Ok(result.matches
        .iter()
        .map(|m| GrepMatch {
            path: path.to_path_buf(),
            line_number: m.line_number,
            context: m.context.clone(),
        })
        .collect())
}