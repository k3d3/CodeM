use crate::WriteError;

pub fn validate_patterns(start_str: &str, end_str: &str) -> Result<(), WriteError> {    
    if start_str.is_empty() || end_str.is_empty() {
        return Err(WriteError::InvalidPatternPair { 
            content: String::from("Start and end patterns cannot be empty")
        });
    }

    // Trim for pattern comparison but keep original strings for matching
    let trimmed_start = start_str.trim();
    let trimmed_end = end_str.trim();
    
    if trimmed_start == trimmed_end {
        return Err(WriteError::InvalidPatternPair {
            content: String::from("Start and end patterns cannot be identical when trimmed")
        });
    }
    
    // Use trimmed patterns for containment checks to avoid whitespace issues
    if trimmed_start.contains(trimmed_end) {
        return Err(WriteError::InvalidPatternPair { 
            content: format!("End pattern '{}' is contained within start pattern '{}', making it ambiguous where the section ends", 
                end_str, start_str) 
        });
    }
    if trimmed_end.contains(trimmed_start) {
        return Err(WriteError::InvalidPatternPair { 
            content: format!("Start pattern '{}' is contained within end pattern '{}', making it ambiguous where the section starts", 
                start_str, end_str) 
        });
    }

    Ok(())
}

pub fn validate_matches(
    start_matches: &[(usize, &str)],
    end_matches: &[(usize, &str)],
    file_content: &str,
) -> Result<(), WriteError> {
    if start_matches.is_empty() {
        return Err(WriteError::StartPatternNotFound { 
            content: file_content.to_string() 
        });
    }
    if start_matches.len() > 1 {
        return Err(WriteError::MultipleStartPatternsFound { 
            content: file_content.to_string() 
        });
    }
    if end_matches.is_empty() {
        return Err(WriteError::EndPatternNotFound { 
            content: file_content.to_string() 
        });
    }
    if end_matches.len() > 1 {
        return Err(WriteError::MultipleEndPatternsFound { 
            content: file_content.to_string() 
        });
    }

    // Check if any end pattern comes before the start pattern
    let (start_pos, _) = start_matches[0];
    if end_matches.iter().any(|(end_pos, _)| end_pos < &start_pos) {
        return Err(WriteError::EndPatternBeforeStart { 
            content: file_content.to_string() 
        });
    }

    Ok(())
}