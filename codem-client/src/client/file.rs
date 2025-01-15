use codem_core::directory::list_directory as core_list_directory;
use codem_core::types::{ListOptions, TreeEntry};
use std::path::Path;

use tokio::io;

pub(crate) async fn list_directory(
    path: impl AsRef<Path>,
    options: ListOptions
) -> io::Result<TreeEntry> {
    let path = path.as_ref();
    let result = core_list_directory(path, path, &options).await?;
    Ok(result)
}
