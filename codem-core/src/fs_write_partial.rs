use crate::types::{
    MatchInfo, PartialWrite, PartialWriteResult, ChangeResult, WriteResult,
};
use crate::WriteError;
use aho_corasick::AhoCorasick;
use std::path::Path;
use tokio::fs;

pub async fn process_partial_write(
    path: &Path,
    partial_writes: PartialWrite,
) -> Result<WriteResult, WriteError> {
    let ac = AhoCorasick::new(partial_writes.changes.iter().map(|pw| pw.old_str.as_bytes()))?;
    let contents = fs::read_to_string(path).await?;
    let lines: Vec<&str> = contents.lines().collect();

    // Get all matches and their positions
    let matches: Vec<MatchInfo> = ac
        .find_iter(&contents)
        .scan(0, |last_match_end, mat| {
            let relative_match_start = mat.start() - *last_match_end;
            *last_match_end = mat.end();
            
            Some(MatchInfo {
                pattern_index: mat.pattern().as_usize(),
                relative_match_start,
            })
        })
        .collect();

    let mut output = String::new();
    let mut last_match_end_position = 0;
    let mut matches_out = Vec::new();

    // Build a map of byte positions to line numbers
    let mut line_start = 0;
    let mut line_map = Vec::new();
    for line in contents.lines() {
        let line_end = line_start + line.len();
        line_map.push((line_start, line_end));
        line_start = line_end + 1; // +1 for newline
    }

    for match_info in &matches {
        let pattern_index = match_info.pattern_index;
        let relative_match_start = match_info.relative_match_start;
        let write = &partial_writes.changes[pattern_index];

        // Write text before match
        let preceding_text = &contents[last_match_end_position..last_match_end_position + relative_match_start];
        output.push_str(preceding_text);

        // Find line number of match
        let match_pos = last_match_end_position + relative_match_start;
        let line_num = line_map
            .iter()
            .position(|(start, end)| match_pos >= *start && match_pos < *end)
            .unwrap();

        // Get context before and after
        let context_start = line_num.saturating_sub(partial_writes.context_lines);
        let context_end = std::cmp::min(line_num + partial_writes.context_lines + 1, lines.len());

        // Build context
        let mut context_lines = Vec::with_capacity(context_end - context_start);
        
        // Add lines before the match
        for &line in &lines[context_start..line_num] {
            context_lines.push(line.to_string());
        }

        // Add the modified line
        let line = lines[line_num];
        let match_rel_pos = match_pos - line_map[line_num].0;
        let mut modified_line = line[..match_rel_pos].to_string();
        modified_line.push_str(&write.new_str);
        modified_line.push_str(&line[match_rel_pos + write.old_str.len()..]);
        
        context_lines.push(modified_line);

        // Add lines after the match
        for &line in &lines[line_num + 1..context_end] {
            context_lines.push(line.to_string());
        }

        let context = context_lines.join("\n");

        // Write the replacement and track line numbers
        output.push_str(&write.new_str);
        let new_lines = write.new_str.chars().filter(|&c| c == '\n').count();

        matches_out.push(ChangeResult {
            partial_write_index: pattern_index,
            line_number_start: line_num + 1,
            line_number_end: line_num + 1 + new_lines,
            context,
        });

        last_match_end_position += relative_match_start + write.old_str.len();
    }

    // Write remaining text
    output.push_str(&contents[last_match_end_position..]);

    // Write file and return result
    fs::write(path, &output).await?;

    Ok(WriteResult {
        line_count: output.lines().count(),
        size: output.len(),
        partial_write_result: Some(PartialWriteResult {
            change_results: matches_out,
            full_content: if partial_writes.return_full_content {
                Some(output)
            } else {
                None
            },
        }),
        partial_write_large_result: None,
    })
}
