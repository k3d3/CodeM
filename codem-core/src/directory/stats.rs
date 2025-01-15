use crate::types::FileMetadata;
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt};
use std::path::Path;

pub async fn get_stats(path: &Path, count_lines: bool) -> io::Result<FileMetadata> {
    let metadata = fs::metadata(path).await?;
    let modified = metadata.modified().ok();
    let size = Some(metadata.len());

    let line_count = if count_lines {
        let file = fs::File::open(path).await?;
        let reader = io::BufReader::new(file);
        let mut lines = 0;
        let mut reader = reader.lines();
        
        while reader.next_line().await?.is_some() {
            lines += 1;
        }
        
        Some(lines)
    } else {
        None
    };

    Ok(FileMetadata {
        modified,
        size,
        line_count,
    })
}