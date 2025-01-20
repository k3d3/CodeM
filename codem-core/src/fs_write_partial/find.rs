use aho_corasick::AhoCorasick;
use crate::types::{MatchInfo, PartialWrite, GrepMatch};
use crate::WriteError;
use std::collections::HashSet;

fn find_line_number(lines: &[&str], pos: usize) -> Option<usize> {
    let mut current_pos = 0;
    for (i, line) in lines.iter().enumerate() {
        current_pos += line.len() + 1; // +1 for newline
        if current_pos > pos {
            return Some(i + 1); // 1-based line numbers like grep
        }
    }
    None
}

fn format_match_context(contents: &str, match_start: usize, pattern: &str, context_lines: usize) -> GrepMatch {
    let lines: Vec<&str> = contents.lines().collect();
    
    // Find line number (1-based)
    let match_line = find_line_number(&lines, match_start)
        .unwrap_or(1);
    
    // Calculate context range
    let context_start = (match_line - 1).saturating_sub(context_lines);
    let context_end = usize::min(match_line - 1 + pattern.lines().count() + context_lines, lines.len());

    let context = lines[context_start..context_end].join("\n");

    GrepMatch {
        line_number: match_line,
        context,
    }
}

pub fn find_matches(
    contents: &str,
    partial_writes: &PartialWrite,
) -> Result<Vec<MatchInfo>, WriteError> {
    let ac = AhoCorasick::new(partial_writes.changes.iter().map(|pw| pw.old_str.as_bytes()))?;

    let all_matches: Vec<_> = ac
        .find_iter(contents)
        .scan(0, |last_match_end, mat| {
            let relative_match_start = mat.start() - *last_match_end;
            *last_match_end = mat.end();
            
            Some(MatchInfo {
                pattern_index: mat.pattern().as_usize(),
                relative_match_start,
            })
        })
        .collect();

    // Get set of patterns that were matched
    let matched_patterns: HashSet<_> = all_matches.iter()
        .map(|m| m.pattern_index)
        .collect();

    // Check if any required pattern wasn't found
    for i in 0..partial_writes.changes.len() {
        if !matched_patterns.contains(&i) {
            return Err(WriteError::PatternNotFound {
                index: i,
                old_str: partial_writes.changes[i].old_str.clone(),
                content: contents.to_string()
            });
        }
    }

    // Count matches per pattern
    let mut match_counts = vec![0; partial_writes.changes.len()];
    for match_info in &all_matches {
        match_counts[match_info.pattern_index] += 1;
    }

    // Check for multiple matches that aren't allowed
    for (i, count) in match_counts.iter().enumerate() {
        if *count > 1 && !partial_writes.changes[i].allow_multiple_matches {
            let pattern = &partial_writes.changes[i].old_str;
            let mut context = String::new();
            let mut first = true;
            
            // Add context for each match
            for match_info in all_matches.iter().filter(|m| m.pattern_index == i) {
                let grep_match = format_match_context(
                    contents,
                    match_info.relative_match_start,
                    pattern,
                    2 // Use 2 lines of context like grep_codebase
                );
                
                if first {
                    context.push_str(&format!("Match on line {}:\n{}", grep_match.line_number, grep_match.context));
                    first = false;
                } else {
                    context.push_str(&format!("\n\nMatch on line {}:\n{}", grep_match.line_number, grep_match.context));
                }
            }

            return Err(WriteError::MultiplePatternMatches { 
                index: i,
                old_str: pattern.clone(),
                content: contents.to_string(),
                match_count: *count,
                context
            });
        }
    }

    Ok(all_matches)
}