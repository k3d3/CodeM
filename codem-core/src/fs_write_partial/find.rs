use aho_corasick::AhoCorasick;
use crate::types::{MatchInfo, PartialWrite};
use crate::WriteError;

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

    // Filter matches based on allow_multiple_matches
    let mut filtered_matches = Vec::new();
    let mut used_patterns = std::collections::HashSet::new();

    for match_info in all_matches {
        let pattern_index = match_info.pattern_index;
        if !used_patterns.contains(&pattern_index) {
            filtered_matches.push(match_info.clone());
            if !partial_writes.changes[pattern_index].allow_multiple_matches {
                used_patterns.insert(pattern_index);
            }
        }
    }

    Ok(filtered_matches)
}