pub fn build_context_lines(
    lines: &[&str],
    line_map: &[(usize, usize)],
    line_num: usize,
    match_pos: usize,
    old_str: &str,
    new_str: &str,
    context_lines: usize,
) -> String {
    // For empty content or single lines with no context needed
    if lines.is_empty() || (lines.len() == 1 && context_lines == 0) {
        let line = if lines.is_empty() { "" } else { lines[0] };
        let match_rel_pos = if match_pos < line_map[0].1 {
            match_pos - line_map[0].0 
        } else { 
            0 
        };
        let mut modified_line = line[..match_rel_pos].to_string();
        modified_line.push_str(new_str.trim_end());
        if match_rel_pos + old_str.trim_end().len() <= line.len() {
            modified_line.push_str(&line[match_rel_pos + old_str.trim_end().len()..]);
        }
        return modified_line;
    }

    // Calculate context boundaries
    let context_start = line_num.saturating_sub(context_lines);
    let context_end = std::cmp::min(line_num + context_lines + 1, lines.len());
    
    let mut context_lines_vec = Vec::with_capacity(context_end.saturating_sub(context_start));

    // Add lines before the match
    if context_start < line_num {
        for &line in &lines[context_start..line_num] {
            context_lines_vec.push(line.to_string());
        }
    }

    // Add the modified line
    if line_num < lines.len() {
        let line = lines[line_num];
        let start_pos = line_map[line_num].0;
        let match_rel_pos = if match_pos >= start_pos {
            match_pos.saturating_sub(start_pos)
        } else {
            0
        };
        
        let mut modified_line = String::new();
        if match_rel_pos > 0 && match_rel_pos <= line.len() {
            modified_line.push_str(&line[..match_rel_pos]);
        }
        modified_line.push_str(new_str.trim_end());
        if match_rel_pos + old_str.trim_end().len() < line.len() {
            modified_line.push_str(&line[match_rel_pos + old_str.trim_end().len()..]);
        }
        context_lines_vec.push(modified_line);
    }

    // Add lines after the match
    if line_num + 1 < lines.len() && line_num + 1 < context_end {
        for &line in &lines[line_num + 1..context_end] {
            context_lines_vec.push(line.to_string());
        }
    }

    context_lines_vec.join("\n")
}