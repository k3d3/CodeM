use crate::types::{WriteResult, PartialWriteLarge, PartialWriteLargeResult, LargeChangeContext};

pub fn create_result(
    start_line: usize, 
    end_line: usize,
    output: &str,
    lines: &[&str],
    partial_write: &PartialWriteLarge,
) -> WriteResult {
    WriteResult {
        line_count: output.lines().count(),
        size: output.len(),
        partial_write_result: None,
        partial_write_large_result: Some(PartialWriteLargeResult {
            line_number_start: start_line + 1,
            line_number_end: end_line + 1,
            context: create_context(lines, start_line, end_line, partial_write),
        }),
    }
}

pub fn create_context(
    lines: &[&str],
    start_line: usize,
    end_line: usize,
    partial_write: &PartialWriteLarge,
) -> LargeChangeContext {
    LargeChangeContext {
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
    }
}

pub fn find_line_numbers(contents: &str, start_pos: usize, end_pos: usize) -> (usize, usize) {
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
        line_start = line_end + 1;
    }

    (
        start_line.unwrap_or(current_line),
        end_line.unwrap_or(current_line),
    )
}