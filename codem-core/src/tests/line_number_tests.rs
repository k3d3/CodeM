use super::*;
use std::fs;
use tempfile::TempDir;
use crate::{fs_write_partial::find::{get_line_number, is_within_line_range}, types::LineRange};

#[test]
fn test_line_number_calculation() {
    // Test with various positions in a multiline string
    let content = "Line 1\nLine 2\nLine 3\nLine 4\n";
    
    let tests = vec![
        (0, 1, "Start of Line 1"),
        (5, 1, "Middle of Line 1"),
        (6, 1, "Newline after Line 1"),
        (7, 2, "Start of Line 2"),
        (13, 2, "Newline after Line 2"),
        (14, 3, "Start of Line 3"),
        (20, 3, "Newline after Line 3"),
        (21, 4, "Start of Line 4")
    ];
    
    for (pos, expected_line, description) in tests {
        assert_eq!(get_line_number(content, pos), expected_line, 
            "Position {} ({}): expected line {}, got line {}", 
            pos, description, expected_line, get_line_number(content, pos));
    }
}

#[test]
fn test_partial_line_position() {
    let content = "Line 1 has pattern\nLine 2 has pattern\nLine 3\n";
    
    // Get actual positions of "pattern" occurrences
    let first_pattern_pos = content.find("pattern").unwrap();
    let second_pattern_pos = content[first_pattern_pos + 1..].find("pattern").unwrap() + first_pattern_pos + 1;
    
    println!("Content:\n{}", content);
    println!("First 'pattern' at pos {}", first_pattern_pos);
    println!("Second 'pattern' at pos {}", second_pattern_pos);
    
    // Test positions inside each instance of "pattern"
    assert_eq!(get_line_number(content, first_pattern_pos), 1, 
        "First 'pattern' at pos {} should be on line 1", first_pattern_pos);
    assert_eq!(get_line_number(content, second_pattern_pos), 2, 
        "Second 'pattern' at pos {} should be on line 2", second_pattern_pos);
}

#[test]
fn test_line_range_with_partial_positions() {
    let content = "Line 1 has pattern\nLine 2 has pattern\nLine 3\n";
    
    let range = Some(LineRange {
        start: Some(2),
        end: Some(2),
    });
    
    // Find actual positions of "pattern" occurrences
    let first_pattern_pos = content.find("pattern").unwrap();
    let second_pattern_pos = content[first_pattern_pos + 1..].find("pattern").unwrap() + first_pattern_pos + 1;
    
    println!("Content:\n{}", content);
    println!("First 'pattern' at pos {}", first_pattern_pos);
    println!("Second 'pattern' at pos {}", second_pattern_pos);
    
    // First "pattern" should be outside range (in line 1)
    let first_line = get_line_number(content, first_pattern_pos);
    println!("First pattern on line {}", first_line);
    assert!(!is_within_line_range(first_line, &range), 
        "First 'pattern' at pos {} (line {}) should be outside range {:?}", 
        first_pattern_pos, first_line, range);
    
    // Second "pattern" should be inside range (in line 2)
    let second_line = get_line_number(content, second_pattern_pos);
    println!("Second pattern on line {}", second_line);
    assert!(is_within_line_range(second_line, &range),
        "Second 'pattern' at pos {} (line {}) should be inside range {:?}",
        second_pattern_pos, second_line, range);
}

#[test]
fn test_line_count_at_boundaries() {
    let content = "Line 1\nLine 2\nLine 3\n";
    
    // Test every position to ensure line counts are correct
    for i in 0..content.len() {
        let line = get_line_number(content, i);
        println!("Position {} (char '{}'): line {}", 
            i, 
            content.chars().nth(i).unwrap_or('?'),
            line);
            
        // Expected lines:
        // "Line 1\n" - everything up to \n is line 1
        // "Line 2\n" - everything up to next \n is line 2
        // "Line 3\n" - everything up to final \n is line 3
        let expected = if i <= 6 { 1 }          // Up to and including first \n
                      else if i <= 13 { 2 }      // Up to and including second \n  
                      else if i <= 20 { 3 }      // Up to and including third \n
                      else { 3 };                // After final newline
                      
        assert_eq!(line, expected, 
            "Wrong line number {} for position {} (char '{}'), expected {}", 
            line, i, content.chars().nth(i).unwrap_or('?'), expected);
    }
}