use std::path::Path;
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