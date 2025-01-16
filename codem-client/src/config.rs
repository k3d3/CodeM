use std::collections::HashMap;
use std::sync::Arc;
use regex::Regex;
use crate::Project;

#[derive(Debug)]
pub struct CommandPattern {
    pub pattern: String,
    pub is_regex: bool,
}

#[derive(Debug)]
pub struct ClientConfig {
    pub projects: HashMap<String, Arc<Project>>,
    pub safe_commands: Vec<CommandPattern>,
    pub risky_commands: Vec<CommandPattern>,
}

impl ClientConfig {
    pub fn new(
        projects: HashMap<String, Arc<Project>>,
        safe_commands: Vec<CommandPattern>,
        risky_commands: Vec<CommandPattern>,
    ) -> Self {
        Self {
            projects,
            safe_commands,
            risky_commands,
        }
    }

    pub fn is_command_safe(&self, command: &str) -> bool {
        // Command is unsafe if it matches any risky pattern
        for pattern in &self.risky_commands {
            let matches = if pattern.is_regex {
                Regex::new(&pattern.pattern)
                    .map(|re| re.is_match(command))
                    .unwrap_or(false)
            } else {
                command.contains(&pattern.pattern)
            };
            if matches {
                return false;
            }
        }

        // Command is safe if it matches any safe pattern
        self.safe_commands.iter().any(|pattern| {
            if pattern.is_regex {
                Regex::new(&pattern.pattern)
                    .map(|re| re.is_match(command))
                    .unwrap_or(false)
            } else {
                command.contains(&pattern.pattern)
            }
        })
    }
}