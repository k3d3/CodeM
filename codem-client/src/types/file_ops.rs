use std::path::PathBuf;
use codem_core::types::{
    PartialWrite, PartialWriteLarge, WriteResult as CoreWriteResult,
    WriteResultDetails as CoreWriteResultDetails,
};

#[derive(Debug)]
pub enum WriteOperation {
    Full(String),
    Partial(PartialWrite),
    PartialLarge(PartialWriteLarge),
}

#[derive(Debug)]
pub struct WriteResultWithChecks {
    pub line_count: usize,
    pub size: usize,
    pub details: CoreWriteResultDetails,
    pub check_results: Vec<String>,
}

impl From<CoreWriteResult> for WriteResultWithChecks {
    fn from(result: CoreWriteResult) -> Self {
        Self {
            line_count: result.line_count,
            size: result.size,
            details: result.details,
            check_results: vec![],
        }
    }
}

#[derive(Debug)]
pub struct GrepMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub context: String,
}