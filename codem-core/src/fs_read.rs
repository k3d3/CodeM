use crate::types::FileMetadata;
use std::path::Path;
use tokio::{fs, io};

#[derive(Default)]
pub struct ReadOptions {
    pub count_lines: bool,
}

pub async fn read_file(
    path: impl AsRef<Path>,
    options: ReadOptions,
) -> io::Result<(String, FileMetadata)> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).await?;
    let metadata = path.metadata()?;

    let line_count = if options.count_lines {
        Some(content.lines().count())
    } else {
        None
    };

    Ok((
        content,
        FileMetadata {
            modified: metadata.modified()?,
            size: metadata.len(),
            line_count,
        },
    ))
}

pub fn get_metadata(path: impl AsRef<Path>, options: ReadOptions) -> io::Result<FileMetadata> {
    let path = path.as_ref();
    let metadata = path.metadata()?;

    let line_count = if options.count_lines {
        Some(std::fs::read_to_string(path)?.lines().count())
    } else {
        None
    };

    Ok(FileMetadata {
        modified: metadata.modified()?,
        size: metadata.len(),
        line_count,
    })
}