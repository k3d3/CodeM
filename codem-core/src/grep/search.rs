use std::path::Path;
use futures::stream::{FuturesUnordered, StreamExt};
use ignore::WalkBuilder;
use tokio::io;
use regex::Regex;
use regex::RegexBuilder;
use std::sync::Arc;

use crate::types::{GrepFileMatch, GrepOptions};
use super::processor::grep_file;

struct SearchContext {
    options: GrepOptions,
    pattern: Regex,
}

pub async fn grep_codebase(
    root: impl AsRef<Path>, 
    pattern: &Regex,
    options: &GrepOptions,
) -> io::Result<Vec<GrepFileMatch>> {
    let mut matches = Vec::new();
    let max_concurrent = num_cpus::get();
    
    let mut walker_builder = WalkBuilder::new(root.as_ref());
    walker_builder
        .ignore(true)  // Use ignore patterns from .gitignore
        .git_global(true) // Use global gitignore
        .git_ignore(true) // Use .gitignore files
        .follow_links(false) // Never follow symlinks for safety
        .require_git(false); // Don't require being in a git repo
        
    #[cfg(test)]
    {
        eprintln!("Walking directory: {:?}", root.as_ref());
    }
    
    let walker = walker_builder.build();
    
    // Build pattern with proper case sensitivity - case insensitive by default
    let context_pattern = RegexBuilder::new(pattern.as_str())
        .case_insensitive(!options.case_sensitive)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    
    let context = Arc::new(SearchContext {
        options: options.clone(),
        pattern: context_pattern,
    });
    
    let mut futures = FuturesUnordered::new();

    // Process all the filesystem entries 
    for result in walker {
        match result {
            Ok(entry) => {
                #[cfg(test)]
                {
                    eprintln!("Entry path: {:?}", entry.path());
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            eprintln!("Content: {}", content);
                        }
                    }
                }
                
                // Skip directories
                if !entry.file_type().map_or(false, |ft| ft.is_file()) {
                    continue;
                }

                // We own the path
                let path = entry.into_path();

                // Apply file pattern filter if specified
                if let Some(pat) = &context.options.file_pattern {
                    let file_name = path.file_name().map(|s| s.to_string_lossy()).unwrap_or_default();
                    if !glob::Pattern::new(pat).map_or(false, |p| p.matches(&file_name)) {
                        continue;
                    }
                }

                if futures.len() >= max_concurrent {
                    if let Some(Ok(Some(result))) = futures.next().await {
                        matches.push(result);
                    }
                }

                let context = Arc::clone(&context);
                futures.push(async move {
                    grep_file(path, &context.pattern, &context.options).await
                });
            }
            Err(err) => {
                eprintln!("Error walking directory: {}", err);
            }
        }
    }

    // Drain remaining futures
    while let Some(result) = futures.next().await {
        if let Ok(Some(file_match)) = result {
            matches.push(file_match);
        }
    }

    Ok(matches)
}