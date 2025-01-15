pub fn build_line_map(contents: &str) -> Vec<(usize, usize)> {
    let mut line_map = Vec::new();
    let mut line_start = 0;
    
    for line in contents.lines() {
        let line_end = line_start + line.len();
        line_map.push((line_start, line_end));
        line_start = line_end + 1; // +1 for newline
    }
    
    line_map
}

pub fn find_line_number(line_map: &[(usize, usize)], pos: usize) -> Option<usize> {
    line_map
        .iter()
        .position(|(start, end)| pos >= *start && pos < *end)
}