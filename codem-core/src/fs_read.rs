use std::fs;
use std::path::Path;
use crate::types::FileMetadata;
use crate::WriteError;
use std::io;

#[derive(Default)]
pub struct ReadOptions {
    pub content_only: bool,
    pub count_lines: bool,
}

pub async fn read_file(
    path: impl AsRef<Path>,
    options: ReadOptions,
) -> io::Result<(String, FileMetadata)> {
    let metadata = get_metadata(&path)?;
    let content = fs::read_to_string(path.as_ref())?;
    
    let line_count = if options.count_lines {
        Some(content.lines().count())
    } else {
        None
    };

    Ok((content, FileMetadata {
        modified: Some(metadata.modified()?),
        size: Some(metadata.len()),
        line_count,
    }))
}

pub fn get_metadata(path: impl AsRef<Path>) -> io::Result<fs::Metadata> {
    fs::metadata(path)
}

pub fn read_with_line_count(path: impl AsRef<Path>) -> Result<(String, FileMetadata), WriteError> {
    let metadata = fs::metadata(path.as_ref())?;
    let content = fs::read_to_string(path.as_ref())?;
    let line_count = Some(content.lines().count());
    
    Ok((
        content,
        FileMetadata {
            modified: metadata.modified().ok(),
            size: Some(metadata.len()),
            line_count,
        }
    ))
}