use regex::Regex;
use std::path::Path;
use std::pin::Pin;
use std::future::Future;

use crate::error::GrepError;
use crate::error::grep_error::Pattern;
use crate::types::{GrepMatch, GrepResults};

pub(crate) async fn grep_file(
    path: impl AsRef<Path>,
    pattern: &str,
) -> Result<Vec<GrepMatch>, GrepError> {
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

    let mut matches = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if regex.is_match(line) {
            matches.push(GrepMatch {
                line_number: line_number + 1,
                line: line.to_string(),
                file_path: path.display().to_string(),
            });
        }
    }

    Ok(matches)
}

pub(crate) async fn grep_codebase(
    root_dir: impl AsRef<Path>,
    pattern: &str,
) -> Result<GrepResults, GrepError> {
    let root_dir = root_dir.as_ref();
    
    if !root_dir.exists() {
        return Err(GrepError::DirectoryNotFound {
            path: root_dir.display().to_string()
        });
    }

    let regex = Regex::new(pattern).map_err(|_| {
        GrepError::InvalidPattern(Pattern(pattern.to_string()))
    })?;

    let mut matches = Vec::new();
    let mut files_searched = 0;
    let mut lines_searched = 0;

    walk_dir(root_dir, &regex, &mut matches, &mut files_searched, &mut lines_searched).await?;

    Ok(GrepResults {
        pattern: pattern.to_string(),
        matches,
        files_searched,
        lines_searched,
    })
}

fn walk_dir<'a>(
    dir: &'a Path,
    regex: &'a Regex,
    matches: &'a mut Vec<GrepMatch>,
    files_searched: &'a mut usize,
    lines_searched: &'a mut usize,
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
                    *files_searched += 1;

                    if let Ok(contents) = tokio::fs::read_to_string(&path).await {
                        for (line_number, line) in contents.lines().enumerate() {
                            *lines_searched += 1;
                            if regex.is_match(line) {
                                matches.push(GrepMatch {
                                    line_number: line_number + 1,
                                    line: line.to_string(),
                                    file_path: path.display().to_string(),
                                });
                            }
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
                    walk_dir(&path, regex, matches, files_searched, lines_searched).await?;
                }
            }
        }

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use tempfile::tempdir;
    use std::fs;

    #[rstest]
    #[tokio::test]
    async fn test_grep_file_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test line 1\nfound this\ntest line 3").unwrap();

        let matches = grep_file(&file_path, "found").await.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line_number, 2);
        assert_eq!(matches[0].line, "found this");
    }

    #[rstest]
    #[tokio::test]
    async fn test_grep_file_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.txt");

        let result = grep_file(&file_path, "pattern").await;
        assert!(matches!(result,
            Err(GrepError::FileNotFound { path }) if path.contains("nonexistent.txt")
        ));
    }

    #[rstest]
    #[tokio::test]
    async fn test_grep_file_invalid_pattern() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test line").unwrap();

        let result = grep_file(&file_path, "*invalid)").await;
        assert!(matches!(result,
            Err(GrepError::InvalidPattern(_))
        ));
    }

    #[rstest]
    #[tokio::test]
    async fn test_grep_codebase() {
        let dir = tempdir().unwrap();
        
        fs::create_dir(dir.path().join("subdir")).unwrap();
        fs::write(
            dir.path().join("file1.txt"),
            "test line 1\nfound this\ntest line 3"
        ).unwrap();
        fs::write(
            dir.path().join("subdir/file2.txt"), 
            "another line\nfound that\nlast line"
        ).unwrap();

        let results = grep_codebase(dir.path(), "found").await.unwrap();
        assert_eq!(results.matches.len(), 2);
        assert_eq!(results.files_searched, 2);
        assert!(results.lines_searched >= 6);

        let mut matches = results.matches;
        matches.sort_by_key(|m| m.file_path.clone());

        assert_eq!(matches[0].line_number, 2);
        assert_eq!(matches[0].line, "found this");
        assert_eq!(matches[1].line_number, 2);
        assert_eq!(matches[1].line, "found that");
    }

    #[rstest]
    #[tokio::test]
    async fn test_grep_codebase_skip_binary() {
        let dir = tempdir().unwrap();
        
        fs::write(
            dir.path().join("test.txt"),
            "found this"
        ).unwrap();
        fs::write(
            dir.path().join("test.exe"), 
            "found that"
        ).unwrap();

        let results = grep_codebase(dir.path(), "found").await.unwrap();
        assert_eq!(results.matches.len(), 1);
        assert_eq!(results.files_searched, 1);
        assert!(results.matches[0].line == "found this");
    }
}