use crate::types::{
    PartialWriteLarge, WriteResult, PartialWriteLargeResult, LargeChangeContext,
};
use crate::WriteError;
use std::path::Path;
use tokio::fs;

pub async fn process_large_partial_write(
    path: &Path,
    partial_write: PartialWriteLarge,
) -> Result<WriteResult, WriteError> {
    let contents = fs::read_to_string(path).await?;
    let lines: Vec<&str> = contents.lines().collect();

    // Check for overlapping patterns (removing newlines for comparison)
    let start_str = partial_write.start_str.trim_end();
    let end_str = partial_write.end_str.trim_end();
    
    if start_str.contains(end_str) || end_str.contains(start_str) {
        return Err(WriteError::InvalidPatternPair);
    }

    // First find all occurrences to check validity
    let start_matches: Vec<_> = contents.match_indices(&partial_write.start_str).collect();
    let end_matches: Vec<_> = contents.match_indices(&partial_write.end_str).collect();

    if start_matches.is_empty() {
        return Err(WriteError::StartPatternNotFound);
    }
    if start_matches.len() > 1 {
        return Err(WriteError::MultipleStartPatternsFound);
    }
    if end_matches.is_empty() {
        return Err(WriteError::EndPatternNotFound);
    }
    if end_matches.len() > 1 {
        return Err(WriteError::MultipleEndPatternsFound);
    }

    let (start_pos, _) = start_matches[0];
    let (end_pos, _) = end_matches[0];

    // Get line numbers for each pattern
    let mut current_line = 0;
    let mut line_start = 0;
    let mut start_line = None;
    let mut end_line = None;

    for line in contents.lines() {
        let line_end = line_start + line.len();

        if start_pos >= line_start && start_pos < line_end {
            start_line = Some(current_line);
        }
        if end_pos >= line_start && end_pos < line_end {
            end_line = Some(current_line);
        }

        current_line += 1;
        line_start = line_end + 1; // +1 for newline
    }

    let start_line = match start_line {
        Some(line) => line,
        None => current_line,
    };

    let end_line = match end_line {
        Some(line) => line,
        None => current_line,
    };

    // Check for nested or overlapping patterns
    let start_str = &partial_write.start_str;
    let end_str = &partial_write.end_str;
    
    // Check if patterns overlap or are nested
    if start_str.contains(end_str) || end_str.contains(start_str) {
        return Err(WriteError::InvalidPatternPair);
    }

    // Check that end pattern appears after start pattern
    if end_matches[0].0 <= start_matches[0].0 {
        return Err(WriteError::EndPatternBeforeStart);
    }

    // Build output string with replacement
    let mut output = String::with_capacity(contents.len());
    output.push_str(&contents[..start_pos]);
    output.push_str(&partial_write.new_str);
    output.push_str(&contents[end_pos + partial_write.end_str.len()..]);

    // Write file and return result
    fs::write(path, &output).await?;

    Ok(WriteResult {
        line_count: output.lines().count(),
        size: output.len(),
        partial_write_result: None,
        partial_write_large_result: Some(PartialWriteLargeResult {
            line_number_start: start_line + 1,
            line_number_end: end_line + 1,
            context: LargeChangeContext {
                before_start: lines[start_line.saturating_sub(partial_write.context_lines)..start_line]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
                start_content: partial_write.new_str.lines()
                    .take(2)
                    .map(|s| s.to_string())
                    .collect(),
                end_content: partial_write.new_str.lines()
                    .rev()
                    .take(2)
                    .map(|s| s.to_string())
                    .collect(),
                after_end: lines[end_line + 1..std::cmp::min(end_line + 1 + partial_write.context_lines, lines.len())]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
            },
        }),
    })
}