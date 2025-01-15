use crate::WriteError;

pub fn validate_patterns(start_str: &str, end_str: &str) -> Result<(), WriteError> {
    let start_str = start_str.trim_end();
    let end_str = end_str.trim_end();
    
    if start_str.contains(end_str) || end_str.contains(start_str) {
        return Err(WriteError::InvalidPatternPair);
    }

    Ok(())
}

pub fn validate_matches(
    start_matches: &[(usize, &str)],
    end_matches: &[(usize, &str)],
) -> Result<(), WriteError> {
    if start_matches.is_empty() {
        return Err(WriteError::StartPatternNotFound);
    }
    if start_matches.len() > 1 {
        return Err(WriteError::MultipleStartPatternsFound);
    }
    if end_matches.is_empty() {
        return Err(WriteError::EndPatternNotFound);
    }
    if end_matches.len() > 1 {
        return Err(WriteError::MultipleEndPatternsFound);
    }

    if end_matches[0].0 <= start_matches[0].0 {
        return Err(WriteError::EndPatternBeforeStart);
    }

    Ok(())
}