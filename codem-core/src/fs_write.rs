use crate::fs_write_partial::process_partial_write;
use crate::fs_write_large_partial::process_large_partial_write;
use crate::types::{WriteOperation, WriteResult, WriteResultDetails};
use crate::WriteError;
use std::path::Path;
use std::time::SystemTime;
use tokio::fs;

pub async fn write_new_file(
    path: &Path,
    contents: &str,
) -> Result<WriteResult, WriteError> {
    if path.exists() {
        let current_content = fs::read_to_string(path).await?;
        return Err(WriteError::FileExists { 
            content: current_content 
        });
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    fs::write(path, &contents).await?;

    // Re-gather metadata for the written path
    let metadata = fs::metadata(path).await?;

    let result = WriteResult {
        line_count: contents.lines().count(),
        size: contents.len(),
        modified: metadata.modified().unwrap(),
        details: WriteResultDetails::None,
    };
    Ok(result)
}

pub async fn write_file(
    path: &Path,
    operation: WriteOperation,
    match_timestamp: Option<SystemTime>,
) -> Result<WriteResult, WriteError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if let Some(timestamp) = match_timestamp {
        let metadata = path.metadata()?;
        let contents = fs::read_to_string(path).await?;
        if metadata.modified()? != timestamp {
            return Err(WriteError::TimestampMismatch { 
                content: contents 
            });
        }
    }

    match operation {
        WriteOperation::Full(contents) => {
            fs::write(path, &contents).await?;

            // Re-gather metadata for the written path, so we can get the new modified timestamp
            let metadata = fs::metadata(path).await?;

            let result = WriteResult {
                line_count: contents.lines().count(),
                size: contents.len(),
                modified: metadata.modified().unwrap(),
                details: WriteResultDetails::None,
            };
            Ok(result)
        }
        WriteOperation::Partial(partial_writes) => process_partial_write(path, partial_writes).await,
        WriteOperation::PartialLarge(partial_write_large) => {
            process_large_partial_write(path, partial_write_large).await
        }
    }
}