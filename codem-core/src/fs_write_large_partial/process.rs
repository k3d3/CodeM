use crate::types::{PartialWriteLarge, WriteResult, LineRange};
use crate::WriteError;
use std::path::Path;
use tokio::fs;
use super::validation::{validate_patterns, validate_matches};
use super::result::{create_result, find_line_numbers};

fn get_line_range_for_pattern(contents: &str, pos: usize, pattern: &str) -> (usize, usize) {
    let prefix = &contents[..pos];
    let start_line = prefix.chars().filter(|&c| c == '\n').count() + 1;
    let pattern_lines = pattern.chars().filter(|&c| c == '\n').count();
    let end_line = start_line + pattern_lines;
    (start_line, end_line)
}

fn pattern_overlaps_range(start_line: usize, end_line: usize, range: &LineRange) -> bool {
    let range_start = range.start.unwrap_or(1);
    let range_end = range.end.unwrap_or(usize::MAX);
    start_line <= range_end && end_line >= range_start
}



pub async fn process_large_partial_write(
    path: &Path,
    partial_write: PartialWriteLarge,
) -> Result<WriteResult, WriteError> {
    let contents = fs::read_to_string(path).await?;
    let lines: Vec<&str> = contents.lines().collect();

    // Validate patterns before any matching
    validate_patterns(&partial_write.start_str, &partial_write.end_str)?;

    let mut start_matches = Vec::new();
    let mut end_matches = Vec::new();

    let mut next_pos = 0;
    
    // First pass: collect all start positions
    while next_pos < contents.len() {
        if let Some(pos) = contents[next_pos..].find(&partial_write.start_str) {
            let start_pos = next_pos + pos;
            
            // Only consider start pattern if we're within line range
            if let Some(range) = &partial_write.line_range {
                let (line_num, _) = get_line_range_for_pattern(&contents, start_pos, &partial_write.start_str);
                if !pattern_overlaps_range(line_num, line_num, range) {
                    next_pos = start_pos + partial_write.start_str.len();
                    continue;
                }
            }
            
            start_matches.push((start_pos, &contents[start_pos..start_pos + partial_write.start_str.len()]));
            next_pos = start_pos + partial_write.start_str.len();
        } else {
            break;
        }
    }
    
    // Second pass: collect all end positions
    next_pos = 0;
    while next_pos < contents.len() {
        if let Some(pos) = contents[next_pos..].find(&partial_write.end_str) {
            let end_pos = next_pos + pos;
            
            // Only consider end pattern if we're within line range
            if let Some(range) = &partial_write.line_range {
                let (line_num, _) = get_line_range_for_pattern(&contents, end_pos, &partial_write.end_str);
                if !pattern_overlaps_range(line_num, line_num, range) {
                    next_pos = end_pos + partial_write.end_str.len();
                    continue;
                }
            }
            
            end_matches.push((end_pos, &contents[end_pos..end_pos + partial_write.end_str.len()]));
            next_pos = end_pos + partial_write.end_str.len();
        } else {
            break;
        }
    }
    
    if start_matches.is_empty() {
        return Err(WriteError::StartPatternNotFound { content: contents.clone() });
    }
    // Validate all matches
    validate_matches(&start_matches, &end_matches, &contents)?;

    // Get matched positions
    let (start_pos, _) = start_matches[0];
    let (end_pos, _) = end_matches.iter()
        .find(|&&(pos, _)| pos > start_pos)
        .ok_or_else(|| WriteError::EndPatternNotFound {
            content: contents.clone()
        })?;

    let (start_line, end_line) = find_line_numbers(&contents, start_pos, *end_pos);

    // Create the output content
    let mut output = String::with_capacity(contents.len());
    output.push_str(&contents[..start_pos]); 
    
    // Add replacement content
    output.push_str(&partial_write.new_str);
    
    // Add remainder, handling newlines
    let remainder_start = *end_pos + partial_write.end_str.len();
    let remainder = &contents[remainder_start..];
    
    // Skip leading newline from remainder if our output has one or remainder has two
    let remainder = if output.ends_with('\n') && remainder.starts_with('\n') {
        &remainder[1..]
    } else if remainder.starts_with("\n\n") {
        &remainder[1..]
    } else {
        remainder
    };
    
    output.push_str(remainder);
    
    // Ensure trailing newline is preserved
    if contents.ends_with('\n') && !output.ends_with('\n') {
        output.push('\n');
    }

    // Write output and update metadata
    fs::write(path, &output).await?;
    let metadata = fs::metadata(path).await?;

    Ok(create_result(start_line, end_line, &output, &lines, &partial_write, &metadata))
}