use aho_corasick::AhoCorasick;
use crate::types::{MatchInfo, PartialWrite};
use crate::WriteError;
use std::collections::HashSet;

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
            return Err(WriteError::StartPatternNotFound);
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
            return Err(WriteError::MultiplePatternMatches { index: i });
        }
    }

    Ok(all_matches)
}