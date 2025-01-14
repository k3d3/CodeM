use regex::Regex;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::types::{GrepMatch, GrepOptions};

pub fn grep_file(path: impl AsRef<Path>, pattern: &Regex) -> io::Result<Vec<GrepMatch>> {
    let mut content = String::new();
    fs::File::open(&path)?.read_to_string(&mut content)?;

    let mut matches = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        for cap in pattern.find_iter(line) {
            matches.push(GrepMatch {
                path: path.as_ref().to_path_buf(),
                line_number: line_num + 1,
                line_content: line.to_string(),
                match_start: cap.start(),
                match_end: cap.end(),
            });
        }
    }

    Ok(matches)
}

pub fn grep_codebase(
    root: impl AsRef<Path>,
    pattern: &Regex,
    options: GrepOptions,
) -> io::Result<Vec<GrepMatch>> {
    let mut matches = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            matches.extend(grep_codebase(entry.path(), pattern, options.clone())?);
        } else if file_type.is_file() {
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
            matches.extend(grep_file(entry.path(), pattern)?);
        }
    }

    Ok(matches)
}
