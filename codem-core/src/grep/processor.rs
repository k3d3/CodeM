use std::path::Path;
use regex::Regex;
use tokio::fs;
use tokio::io;

use crate::types::{GrepMatch, GrepFileMatch, GrepOptions};
use crate::fs_ops::is_in_git_dir;

pub async fn grep_file(path: impl AsRef<Path>, pattern: &Regex, options: &GrepOptions) -> io::Result<Option<GrepFileMatch>> {
    // Skip files in .git directories
    if is_in_git_dir(&path) {
        return Ok(None);
    }

    let content = fs::read_to_string(path.as_ref()).await?;
    let lines: Vec<&str> = content.lines().collect();
    let mut matches = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        if pattern.is_match(line) {
            let context_start = line_num.saturating_sub(options.context_lines);
            let context_end = usize::min(line_num + options.context_lines + 1, lines.len());
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