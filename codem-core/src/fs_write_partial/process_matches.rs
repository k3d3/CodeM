use crate::types::{MatchInfo, PartialWrite, ChangeResult};

pub fn process_matches(
    contents: &str,
    lines: &[&str],
    line_map: &[(usize, usize)],
    matches: &[MatchInfo],
    partial_writes: &PartialWrite,
) -> (String, Vec<ChangeResult>) {
    let mut output = String::new();
    let mut last_match_end_position = 0;
    let mut matches_out = Vec::new();

    for match_info in matches {
        let pattern_index = match_info.pattern_index;
        let relative_match_start = match_info.relative_match_start;
        let write = &partial_writes.changes[pattern_index];

        let preceding_text = &contents[last_match_end_position..last_match_end_position + relative_match_start];
        output.push_str(preceding_text);

        let match_pos = last_match_end_position + relative_match_start;
        let line_num = super::line_map::find_line_number(line_map, match_pos).unwrap();

        let context = super::context::build_context_lines(
            lines,
            line_map,
            line_num,
            match_pos,
            &write.old_str,
            &write.new_str,
            partial_writes.context_lines,
        );

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

    output.push_str(&contents[last_match_end_position..]);
    (output, matches_out)
}