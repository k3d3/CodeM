use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::ConfigError;
use crate::project::Project;

/// Configuration for the Codem client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Project configurations
    pub(crate) projects: HashMap<String, Arc<Project>>,

    /// Path to the session persistence TOML file  
    pub session_file: PathBuf,
    
    /// Patterns for safe commands that can be executed
    pub safe_patterns: Vec<String>,
    
    /// Patterns for risky commands that require extra confirmation
    pub risky_patterns: Vec<String>,
}

impl ClientConfig {
    /// Create a new client configuration
    ///
    /// # Errors
    /// - Returns `ConfigError::InvalidSessionFile` if the session file path is not valid
    /// - Returns `ConfigError::InvalidProject` if a project is invalid
    /// - Returns `ConfigError::InvalidPattern` if a pattern is invalid
    pub fn new(
        projects: Vec<Project>,
        session_file: PathBuf, 
        safe_patterns: Vec<String>,
        risky_patterns: Vec<String>
    ) -> Result<Self, ConfigError> {
        // Validate session file path
        if !session_file.parent().map_or(false, |p| p.exists()) {
            return Err(ConfigError::InvalidSessionFile { 
                path: session_file
            });
        }

        // Validate patterns
        for pattern in &safe_patterns {
            if pattern.is_empty() {
                return Err(ConfigError::InvalidPattern {
                    pattern: pattern.clone()
                });
            }
        }

        for pattern in &risky_patterns {
            if pattern.is_empty() {
                return Err(ConfigError::InvalidPattern {
                    pattern: pattern.clone() 
                });
            }
        }

        let projects = projects.into_iter()
            .map(|p| (p.name.clone(), Arc::new(p)))
            .collect();

        Ok(Self {
            projects,
            session_file,
            safe_patterns,
            risky_patterns,
        })
    }

    /// Check if a command is safe to execute
    ///
    /// A command is considered safe if:
    /// 1. It doesn't match any risky patterns AND
    /// 2. It matches at least one safe pattern
    pub fn is_command_safe(&self, command: &str) -> bool {
        // If command matches any risky pattern, it's not safe
        if self.risky_patterns.iter().any(|p| command.contains(p)) {
            return false;
        }

        // Command must match at least one safe pattern
        self.safe_patterns.iter().any(|p| command.contains(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use rstest::rstest;

    #[rstest]
    #[case("cargo test", vec!["cargo".into()], vec![], true)]
    #[case("cargo test", vec![], vec!["cargo".into()], false)]
    #[case("cargo test", vec!["cargo".into()], vec!["cargo".into()], false)]
    #[case("big bad command", vec!["big".into()], vec!["big bad".into()], false)]
    fn test_is_command_safe(
        #[case] command: &str,
        #[case] safe_patterns: Vec<String>,
        #[case] risky_patterns: Vec<String>,
        #[case] expected: bool
    ) {
        // Create temp session file directory
        let temp_dir = std::env::temp_dir().join("codem_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let tmp_projects = vec![Project::new(temp_dir.clone())];

        let config = ClientConfig::new(
            tmp_projects,
            temp_dir.join("session.toml"),
            safe_patterns,
            risky_patterns
        ).unwrap();

        assert_eq!(config.is_command_safe(command), expected);

        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test] 
    fn test_invalid_pattern() {
        let temp_dir = std::env::temp_dir().join("codem_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let result = ClientConfig::new(
            vec![Project::new(temp_dir.clone())],
            temp_dir.join("session.toml"),  
            vec!["".to_string()], // Empty pattern
            vec![]
        );

        assert!(matches!(result, Err(ConfigError::InvalidPattern { .. })));
        
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_invalid_session_file() {
        let result = ClientConfig::new(
            vec![],
            PathBuf::from("/nonexistent/dir/session.toml"),
            vec!["test".to_string()],
            vec![]  
        );

        assert!(matches!(result, Err(ConfigError::InvalidSessionFile { .. })));
    }
}