use rstest::rstest;
use crate::session::SessionId;
use std::collections::HashSet;

#[rstest]
fn test_session_id_format() {
    let id = SessionId::new();
    let parts: Vec<&str> = id.as_str().split('-').collect();
    
    assert_eq!(parts.len(), 2, "Session ID should contain exactly one hyphen");
}

#[rstest]
fn test_session_id_components() {
    let id = SessionId::new();
    let parts: Vec<&str> = id.as_str().split('-').collect();

    let adj = parts[0];
    let noun = parts[1];

    assert!(!adj.is_empty(), "Adjective part should not be empty");
    assert!(!noun.is_empty(), "Noun part should not be empty");
    assert_eq!(adj.chars().all(|c| c.is_ascii_lowercase()), true, "Adjective should be lowercase");
    assert_eq!(noun.chars().all(|c| c.is_ascii_lowercase()), true, "Noun should be lowercase");
}

#[rstest]
fn test_repeated_generation() {
    let mut generated = HashSet::new();
    
    // Generate a bunch of IDs and verify they're all valid format
    for _ in 0..100 {
        let id = SessionId::new();
        let parts: Vec<&str> = id.as_str().split('-').collect();
        
        assert_eq!(parts.len(), 2);
        generated.insert(id);
    }
}

#[rstest]
fn test_display_matches_as_str() {
    let id = SessionId::new();
    assert_eq!(id.to_string(), id.as_str());
}