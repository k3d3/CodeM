use crate::WriteError;

pub fn validate_patterns(start_str: &str, end_str: &str) -> Result<(), WriteError> {
    let start_str = start_str.trim_end();
    let end_str = end_str.trim_end();
    
    if start_str.contains(end_str) {
        return Err(WriteError::InvalidPatternPair { 
            content: format!("End pattern '{}' is contained within start pattern '{}', making it ambiguous where the section ends", 
                end_str, start_str) 
        });
    }
    if end_str.contains(start_str) {
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

    if end_matches[0].0 <= start_matches[0].0 {
        return Err(WriteError::EndPatternBeforeStart { 
            content: file_content.to_string() 
        });
    }

    Ok(())
}
