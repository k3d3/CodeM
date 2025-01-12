use crate::types::FileMetadata;
use std::io;
use std::path::Path;

#[derive(Default)]
pub struct ReadOptions {
    pub count_lines: bool,
}

pub fn read_file(path: impl AsRef<Path>, options: ReadOptions) -> io::Result<(String, FileMetadata)> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)?;
    let metadata = path.metadata()?;

    let line_count = if options.count_lines {
        Some(content.lines().count())
    } else {
        None
    };

    Ok((content, FileMetadata {
        modified: metadata.modified()?,
        size: metadata.len(),
        line_count,
    }))
}

pub fn write_file(path: impl AsRef<Path>, content: &str) -> io::Result<FileMetadata> {
    let path = path.as_ref();
    
    // Create directories if they don't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(path, content)?;
    let metadata = path.metadata()?;
    
    Ok(FileMetadata {
        modified: metadata.modified()?,
        size: metadata.len(),
        line_count: None,
    })
}