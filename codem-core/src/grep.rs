use std::path::Path;
use futures::stream::{FuturesUnordered, StreamExt};
use regex::Regex;
use tokio::fs;
use tokio::io;

use crate::types::{GrepMatch, GrepFileMatch, GrepOptions};

pub async fn grep_file(path: impl AsRef<Path>, pattern: &Regex, options: &GrepOptions) -> io::Result<Option<GrepFileMatch>> {
    let content = fs::read_to_string(path.as_ref()).await?;
    let lines: Vec<&str> = content.lines().collect();
    let mut matches = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        if pattern.is_match(line) {
            // Build context string
            let context_start = line_num.saturating_sub(options.context_before);
            let context_end = usize::min(line_num + options.context_after + 1, lines.len());
            let context = lines[context_start..context_end].join("\n");

            matches.push(GrepMatch {
                line_number: line_num + 1,
                context,
            });
        }
    }

    if matches.is_empty() {
        Ok(None)
    } else {
        Ok(Some(GrepFileMatch {
            path: path.as_ref().to_path_buf(),
            matches,
        }))
    }
}

pub async fn grep_codebase(
    root: impl AsRef<Path>,
    pattern: &Regex,
    options: GrepOptions,
) -> io::Result<Vec<GrepFileMatch>> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    
    // Collect all entries first
    let mut read_dir = fs::read_dir(root).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_file() {
            files.push(entry.path());
        } else if entry.file_type().await?.is_dir() {
            dirs.push(entry.path());
        }
    }
    
    // Sort for consistent ordering
    files.sort();
    dirs.sort();
    
    // Filter files based on pattern
    let filtered_files: Vec<_> = files.into_iter()
        .filter(|path| {
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
        })
        .collect();

    let mut matches = Vec::new();
    let max_concurrent = num_cpus::get();

    // Process files concurrently with bounded concurrency
    let mut file_futures = FuturesUnordered::new();
    for path in filtered_files {
        if file_futures.len() >= max_concurrent {
            if let Some(Ok(Some(result))) = file_futures.next().await {
                matches.push(result);
            }
        }
        file_futures.push(grep_file(path, pattern, &options));
    }

    // Finish remaining file futures
    while let Some(Ok(Some(result))) = file_futures.next().await {
        matches.push(result);
    }

    // Process directories concurrently with bounded concurrency
    let mut dir_futures = FuturesUnordered::new();
    for path in dirs {
        if dir_futures.len() >= max_concurrent {
            if let Some(Ok(dir_matches)) = dir_futures.next().await {
                matches.extend(dir_matches);
            }
        }
        dir_futures.push(grep_codebase(path, pattern, options.clone()));
    }

    // Finish remaining directory futures
    while let Some(Ok(dir_matches)) = dir_futures.next().await {
        matches.extend(dir_matches);
    }

    Ok(matches)
}
