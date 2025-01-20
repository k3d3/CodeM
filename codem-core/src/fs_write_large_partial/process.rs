use crate::types::{PartialWriteLarge, WriteResult, LineRange};
use crate::WriteError;
use std::path::Path;
use tokio::fs;
use super::validation::{validate_patterns, validate_matches};
use super::result::{create_result, find_line_numbers};

fn get_line_number(contents: &str, pos: usize) -> usize {
    let prefix = &contents[..pos];
    prefix.lines().count() + 1
}

fn is_within_line_range(line_number: usize, line_range: &Option<LineRange>) -> bool {
    match line_range {
        None => true,
        Some(range) => {
            let after_start = match range.start {
                None => true,
                Some(start) => line_number >= start,
            };
            let before_end = match range.end {
                None => true,
                Some(end) => line_number <= end,
            };
            after_start && before_end
        }
    }
}

pub async fn process_large_partial_write(
    path: &Path,
    partial_write: PartialWriteLarge,
) -> Result<WriteResult, WriteError> {
    let contents = fs::read_to_string(path).await?;
    let lines: Vec<&str> = contents.lines().collect();

    validate_patterns(&partial_write.start_str, &partial_write.end_str)?;

    // Get all matches for both start and end patterns
    let start_matches: Vec<_> = contents.match_indices(&partial_write.start_str)
        .filter(|(pos, _)| {
            let line_num = get_line_number(&contents, *pos);
            is_within_line_range(line_num, &partial_write.line_range)
        })
        .collect();
    let end_matches: Vec<_> = contents.match_indices(&partial_write.end_str)
        .filter(|(pos, _)| {
            let line_num = get_line_number(&contents, *pos);
            is_within_line_range(line_num, &partial_write.line_range)
        })
        .collect();

    validate_matches(&start_matches, &end_matches, &contents)?;

    let (start_pos, _) = start_matches[0];
    let (end_pos, _) = end_matches[0];

    let (start_line, end_line) = find_line_numbers(&contents, start_pos, end_pos);

    let output = replace_content(
        &contents,
        start_pos,
        &partial_write.end_str,
        end_pos,
        &partial_write.new_str,
    );

    fs::write(path, &output).await?;

    // Re-gather metadata for the written path, so we can get the new modified timestamp
    let metadata = fs::metadata(path).await?;

    Ok(create_result(start_line, end_line, &output, &lines, &partial_write, &metadata))
}

fn replace_content(
    contents: &str,
    start_pos: usize,
    end_str: &str,
    end_pos: usize,
    new_str: &str,
) -> String {
    let mut output = String::with_capacity(contents.len());
    output.push_str(&contents[..start_pos]);
    output.push_str(new_str);
    output.push_str(&contents[end_pos + end_str.len()..]);
    output
}