use crate::types::{PartialWriteLarge, WriteResult};
use crate::WriteError;
use std::path::Path;
use tokio::fs;
use super::validation::{validate_patterns, validate_matches};
use super::result::{create_result, find_line_numbers};

pub async fn process_large_partial_write(
    path: &Path,
    partial_write: PartialWriteLarge,
) -> Result<WriteResult, WriteError> {
    let contents = fs::read_to_string(path).await?;
    let lines: Vec<&str> = contents.lines().collect();

    validate_patterns(&partial_write.start_str, &partial_write.end_str)?;

    let start_matches: Vec<_> = contents.match_indices(&partial_write.start_str).collect();
    let end_matches: Vec<_> = contents.match_indices(&partial_write.end_str).collect();

    validate_matches(&start_matches, &end_matches)?;

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