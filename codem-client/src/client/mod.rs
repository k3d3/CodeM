pub(crate) mod grep;
pub(crate) mod file;

use std::path::Path;
use crate::error::GrepError;
use codem_core::types::*;

use tokio::io;

use anyhow::Result;

#[derive(Default)]
pub struct Client {
    // Add client fields as needed
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn read_file(&self, _path: impl AsRef<Path>) -> Result<String> {
        // TODO: Implement read_file
        todo!()
    }

    pub async fn write_file(&self, _path: impl AsRef<Path>, _content: &str) -> Result<()> {
        // TODO: Implement write_file  
        todo!()  
    }

    pub async fn grep_file(
        &self,
        path: impl AsRef<Path>,
        pattern: &str
    ) -> Result<GrepFileMatch, GrepError> {
        grep::grep_file(path, pattern).await
    }

    pub async fn list_directory(
        &self,
        path: impl AsRef<Path>,
        options: ListOptions
    ) -> io::Result<TreeEntry> {
        file::list_directory(path, options).await
    }

    pub async fn grep_codebase(
        &self,
        root_dir: impl AsRef<Path>,
        pattern: &str
    ) -> Result<Vec<GrepFileMatch>, GrepError> {
        grep::grep_codebase(root_dir, pattern).await
    }
}