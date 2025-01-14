use codem_core::types::FileMetadata;

#[derive(Debug)]
pub struct ReadResult {
    pub content: String,
    pub metadata: FileMetadata,
}
