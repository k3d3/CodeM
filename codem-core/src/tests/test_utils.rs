use std::path::PathBuf;
use std::future::Future;
use tempfile::TempDir;

pub async fn with_test_dir<F, Fut>(f: F)
where
    F: FnOnce(PathBuf) -> Fut,
    Fut: Future<Output = ()>,
{
    let dir = TempDir::new().unwrap();
    f(dir.path().to_path_buf()).await;
}