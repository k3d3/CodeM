use regex::Regex;
use std::path::Path;
use std::pin::Pin;
use std::future::Future;

use crate::error::GrepError;
use crate::error::grep_error::Pattern;
use codem_core::types::{GrepMatch, GrepFileMatch};

pub(crate) async fn grep_file(
    path: impl AsRef<Path>, 
    pattern: &str
) -> Result<GrepFileMatch, GrepError> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(GrepError::FileNotFound { 
            path: path.display().to_string() 
        });
    }

    let regex = Regex::new(pattern).map_err(|_| {
        GrepError::InvalidPattern(Pattern(pattern.to_string()))
    })?;

    let contents = tokio::fs::read_to_string(path).await.map_err(|source| {
        GrepError::ReadError {
            path: path.display().to_string(),
            source,
        }
    })?;

    let mut grep_matches = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if regex.is_match(line) {
            grep_matches.push(GrepMatch {
                line_number: line_number + 1,
                context: line.to_string(),
            });
        }
    }
    
    Ok(GrepFileMatch {
        path: path.to_path_buf(),
        matches: grep_matches,
    })


}

pub(crate) async fn grep_codebase(
    root_dir: impl AsRef<Path>,
    pattern: &str,
) -> Result<Vec<GrepFileMatch>, GrepError> {
    let root_dir = root_dir.as_ref();
    
    if !root_dir.exists() {
        return Err(GrepError::DirectoryNotFound {
            path: root_dir.display().to_string()
        });
    }

    let regex = Regex::new(pattern).map_err(|_| {
        GrepError::InvalidPattern(Pattern(pattern.to_string()))
    })?;

    let mut file_matches = Vec::new();
    walk_dir(root_dir, &regex, pattern, &mut file_matches).await?;
    Ok(file_matches)
}

fn walk_dir<'a>(
    dir: &'a Path,
    regex: &'a Regex,
    pattern: &'a str,
    file_matches: &'a mut Vec<GrepFileMatch>,
) -> Pin<Box<dyn Future<Output = Result<(), GrepError>> + 'a>> {
    Box::pin(async move {
        let mut dir_entries = tokio::fs::read_dir(dir).await.map_err(|e| {
            GrepError::SearchFailed(Pattern(format!(
                "Failed to read directory {}: {}", 
                dir.display(), e
            )))
        })?;

        while let Some(entry) = dir_entries.next_entry().await.map_err(|e| {
            GrepError::SearchFailed(Pattern(format!(
                "Failed to read directory entry: {}", e
            )))
        })? {
            let path = entry.path();

            if path.is_file() {
                let is_binary = path
                    .extension()
                    .map(|ext| {
                        let ext = ext.to_string_lossy().to_lowercase();
                        matches!(ext.as_str(), 
                            "exe" | "dll" | "so" | "dylib" | "bin" | "obj" |
                            "o" | "a" | "lib" | "ko" | "pyc" | "pyo"
                        )
                    })
                    .unwrap_or(false);

                if !is_binary {
                    if let Ok(grep_file_match) = grep_file(&path, pattern).await {
                        if !grep_file_match.matches.is_empty() {
                            file_matches.push(grep_file_match);
                        }
                    }
                }
            } else if path.is_dir() {
                let name = path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                if !matches!(name.as_str(), 
                    "target" | "node_modules" | ".git" | 
                    ".idea" | ".vscode" | "bin" | "obj"
                ) {
                    walk_dir(&path, regex, pattern, file_matches).await?;
                }
            }
        }

        Ok(())
    })
}

