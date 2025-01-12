use crate::error::{FileError, Result};
use crate::types::{WriteOperation, WriteResult, file_ops::FileMatch};
use std::path::Path;
use codem_core::{read_file, write_file};

pub(super) async fn handle_operation(path: &Path, operation: WriteOperation) -> Result<WriteResult> {
    let mut result = WriteResult::default();
    
    let current_content = match read_file(path, Default::default()) {
        Ok((content, _)) => content,
        Err(_) => String::new()
    };

    // Always store original content
    result.original_content = Some(current_content.clone());

    match operation {
        WriteOperation::Full(content) => {
            match write_file(path, &content) {
                Ok(_) => {
                    result.matches.push(FileMatch {
                        path: path.to_path_buf(),
                        line_number: 1,
                        content: content.clone(),
                    });
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        WriteOperation::Partial { old_str, new_str } => {
            // First find all occurrences of the pattern
            let matches: Vec<_> = current_content.match_indices(&old_str).collect();
            println!("Found matches: {:?}", matches);

            if matches.is_empty() {
                return Err(FileError::PatternNotFound);
            }

            if matches.len() > 1 {
                return Err(FileError::MultipleMatches);
            }

            // Create new content with replacement
            let new_content = current_content.replacen(&old_str, &new_str, 1);
            
            // Calculate line number
            let first_match = matches[0].0;
            let line_number = current_content[..first_match].lines().count() + 1;

            // Try write
            match write_file(path, &new_content) {
                Ok(_) => {
                    result.matches.push(FileMatch {
                        path: path.to_path_buf(),
                        line_number,
                        content: new_str.clone(),
                    });
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    Ok(result)
}