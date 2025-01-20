use aho_corasick::AhoCorasick;
use crate::types::{MatchInfo, PartialWrite, GrepMatch, LineRange};
use crate::WriteError;
use std::collections::HashSet;

pub fn get_line_number(contents: &str, pos: usize) -> usize {
    // Count newlines up to this position
    let newlines = contents[..pos].chars().filter(|&c| c == '\n').count();
    newlines + 1  // Convert to 1-based line numbers
}

pub fn is_within_line_range(line_number: usize, line_range: &Option<LineRange>) -> bool {
    match line_range {
        None => true,
        Some(range) => {
            let after_start = match range.start {
                None => true,
                Some(start) => line_number >= start,  // line_number is already 1-based
            };
            let before_end = match range.end {
                None => true,
                Some(end) => line_number <= end,
            };
            after_start && before_end
        }
    }
}

fn format_match_context(contents: &str, match_start: usize, _pattern: &str, context_lines: usize) -> GrepMatch {
    let lines: Vec<&str> = contents.lines().collect();
    let match_line = get_line_number(contents, match_start);
    
    // Calculate context range, adjusting for 1-based line numbers
    let context_start = match_line.saturating_sub(context_lines + 1);
    let context_end = usize::min(match_line + context_lines - 1, lines.len());

    let context = if context_start < context_end {
        lines[context_start..context_end].join("\n")
    } else {
        String::new()
    };

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

    // Debug line number checks
    println!("{}", "=".repeat(50));
    println!("Contents:\n{}", contents);
    for (i, mat) in ac.find_iter(contents).enumerate() {
        let line_number = get_line_number(contents, mat.start());
        let change = &partial_writes.changes[mat.pattern().as_usize()];
        println!("\nMatch #{} at position {}, line {}", i + 1, mat.start(), line_number);
        if let Some(range) = &change.line_range {
            println!("Line range: {:?}-{:?}", range.start, range.end);
            let in_range = is_within_line_range(line_number, &change.line_range);
            println!("Line {} in range? {}", line_number, in_range);
        }
    }
    println!("{}", "=".repeat(50));

    let mut all_matches: Vec<_> = ac.find_iter(contents)
        .filter_map(|mat| {
            let line_number = get_line_number(contents, mat.start());
            let change = &partial_writes.changes[mat.pattern().as_usize()];
            let match_pos = mat.start();
            
            // Check if this match falls within range
            let in_range = match &change.line_range {
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
                    println!("Match at pos {}, line {}: after_start={}, before_end={}", 
                        match_pos, line_number, after_start, before_end);
                    after_start && before_end
                }
            };

            if in_range {
                Some(MatchInfo {
                    pattern_index: mat.pattern().as_usize(),
                    relative_match_start: match_pos,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort matches by position
    all_matches.sort_by_key(|m| m.relative_match_start);

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

    println!("Found {} matches after filtering", all_matches.len());
    for (i, m) in all_matches.iter().enumerate() {
        let line_number = get_line_number(contents, m.relative_match_start);
        println!("Match #{} at position {}, line {}", i + 1, m.relative_match_start, line_number);
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