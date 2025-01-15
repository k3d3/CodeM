use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;

use crate::types::{GrepMatch, GrepFileMatch, GrepOptions};

pub fn grep_file(path: impl AsRef<Path>, pattern: &Regex, options: &GrepOptions) -> io::Result<Option<GrepFileMatch>> {
    let content = fs::read_to_string(&path)?;
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

pub fn grep_codebase(
    root: impl AsRef<Path>,
    pattern: &Regex,
    options: GrepOptions,
) -> io::Result<Vec<GrepFileMatch>> {
    let mut file_matches = Vec::new();
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    
    // Collect all entries first, separating files and directories
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry);
        } else if entry.file_type()?.is_dir() {
            dirs.push(entry);
        }
    }
    
    // Sort files and directories by file name
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    dirs.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    
    // Process files in current directory
    for entry in &files {
        if let Some(file_pattern) = &options.file_pattern {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if !glob::Pattern::new(file_pattern)
                .unwrap()
                .matches(&file_name_str)
            {
                continue;
            }
        }
        
        if let Some(file_match) = grep_file(&entry.path(), pattern, &options)? {
            file_matches.push(file_match);
        }
    }

    // Process subdirectories
    for entry in &dirs {
        file_matches.extend(grep_codebase(&entry.path(), pattern, options.clone())?);
    }

    Ok(file_matches)
}