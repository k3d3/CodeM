use std::path::Path;
use futures::stream::{FuturesUnordered, StreamExt};
use regex::Regex;
use tokio::fs;
use tokio::io;

use crate::types::{GrepFileMatch, GrepOptions};
use super::processor::grep_file;

fn matches_file_pattern(path: &Path, options: &GrepOptions) -> bool {
    if let Some(file_pattern) = &options.file_pattern {
        let file_name = path.file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default();
        
        glob::Pattern::new(file_pattern)
            .map(|p| p.matches(&file_name))
            .unwrap_or(false)
    } else {
        true
    }
}

pub async fn grep_codebase(
    root: impl AsRef<Path>,
    pattern: &Regex,
    options: GrepOptions,
) -> io::Result<Vec<GrepFileMatch>> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    
    let mut read_dir = fs::read_dir(root).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_file() {
            files.push(entry.path());
        } else if entry.file_type().await?.is_dir() {
            dirs.push(entry.path());
        }
    }
    
    files.sort();
    dirs.sort();
    
    let filtered_files: Vec<_> = files.into_iter()
        .filter(|path| matches_file_pattern(path, &options))
        .collect();

    let mut matches = Vec::new();
    let max_concurrent = num_cpus::get();

    let mut file_futures = FuturesUnordered::new();
    for path in filtered_files {
        if file_futures.len() >= max_concurrent {
            if let Some(Ok(Some(result))) = file_futures.next().await {
                matches.push(result);
            }
        }
        file_futures.push(grep_file(path, pattern, &options));
    }

    while let Some(Ok(Some(result))) = file_futures.next().await {
        matches.push(result);
    }

    let mut dir_futures = FuturesUnordered::new();
    for path in dirs {
        if dir_futures.len() >= max_concurrent {
            if let Some(Ok(dir_matches)) = dir_futures.next().await {
                matches.extend(dir_matches);
            }
        }
        dir_futures.push(grep_codebase(path, pattern, options.clone()));
    }

    while let Some(Ok(dir_matches)) = dir_futures.next().await {
        matches.extend(dir_matches);
    }

    Ok(matches)
}