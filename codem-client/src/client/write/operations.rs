use crate::{
    types::WriteOperation,
    error::{ClientError, OperationError},
};
use codem_core::{
    fs_write::write_file, 
    types::{WriteResult, WriteOperation as CoreWriteOperation}
};
use std::path::Path;

pub(super) async fn handle_operation(
    path: &Path,
    operation: WriteOperation,
) -> Result<WriteResult, ClientError> {
    // Convert client WriteOperation to core WriteOperation
    let core_op = match operation {
        WriteOperation::Full(content) => CoreWriteOperation::Full(content),
        WriteOperation::Partial(write) => CoreWriteOperation::Partial(write),
        WriteOperation::PartialLarge(write) => CoreWriteOperation::PartialLarge(write),
    };

    // Perform the write operation and map errors
    write_file(path, core_op, None)
        .await
        .map_err(|e| OperationError::from(e).into())
}