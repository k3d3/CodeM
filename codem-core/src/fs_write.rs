use crate::fs_write_partial::process_partial_write;
use crate::fs_write_large_partial::process_large_partial_write;
use crate::types::{WriteOperation, WriteResult, WriteResultDetails};
use crate::WriteError;
use std::path::Path;
use std::time::SystemTime;
use tokio::fs;

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
        if metadata.modified()? != timestamp {
            return Err(WriteError::TimestampMismatch);
        }
    }

    match operation {
        WriteOperation::Full(contents) => {
            fs::write(path, &contents).await?;
            let result = WriteResult {
                line_count: contents.lines().count(),
                size: contents.len(),
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