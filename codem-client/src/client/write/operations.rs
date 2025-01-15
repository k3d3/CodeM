use crate::error::FileError;
use anyhow::Result;
use crate::types::{file_ops::FileMatch, WriteOperation, WriteResult};
use codem_core::fs_read::read_file;
use codem_core::fs_write::write_file;
use std::path::Path;

pub(super) async fn handle_operation(
    path: &Path,
    operation: WriteOperation,
) -> Result<WriteResult> {
    let mut result = WriteResult::default();

    let (current_content, _) = read_file(path, Default::default()).await?;

    // Always store original content
    result.original_content = Some(current_content.clone());

    match operation {
        WriteOperation::Full(content) => {
            write_file(path, WriteOperation::Full(content.clone()), None).await?;
            result.matches.push(FileMatch {
                path: path.to_path_buf(),
                line_number: 1,
                context: content,
            });
        }
        WriteOperation::Partial { old_str, new_str } => {
            // First find all occurrences of the pattern
            let matches: Vec<_> = current_content.match_indices(&old_str).collect();
            println!("Found matches: {:?}", matches);

            if matches.is_empty() {
                return Err(FileError::PatternNotFound.into());
            }

            if matches.len() > 1 {
                return Err(FileError::MultipleMatches.into());
            }

            // Create new content with replacement
            let new_content = current_content.replacen(&old_str, &new_str, 1);

            // Calculate line number
            let first_match = matches[0].0;
            let line_number = current_content[..first_match].lines().count() + 1;

            // Try write
            write_file(path, WriteOperation::Full(new_content), None).await?;
            result.matches.push(FileMatch {
                path: path.to_path_buf(),
                line_number,
                context: new_str,
            });
        }
    }

    Ok(result)
}
