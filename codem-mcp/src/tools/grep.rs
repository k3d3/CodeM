use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use std::path::PathBuf;
use crate::{error::format_error_response, server::Mcp};
use codem_core::types::GrepFileMatch;

pub fn grep_file_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for grepping"
            },
            "path": {
                "type": "string",
                "description": "Path to file to grep (relative to project root)"
            },
            "pattern": {
                "type": "string",
                "description": "Regular expression pattern to search for"
            },
            "case_sensitive": {
                "type": "boolean",
                "description": "Whether to perform case-sensitive matching",
                "default": false
            },
            "context_lines": {
                "type": "integer",
                "description": "Number of context lines to include around matches",
                "default": 2,
                "minimum": 0
            }
        },
        "required": ["session_id", "path", "pattern"]
    })
}

pub fn grep_codebase_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for grepping"
            },
            "path": {
                "type": "string",
                "description": "Path to directory to grep (relative to project root)"
            },
            "file_pattern": {
                "type": "string",
                "description": "Optional regex pattern to filter files to search"
            },
            "pattern": {
                "type": "string",
                "description": "Regex pattern to search for"
            },
            "case_sensitive": {
                "type": "boolean",
                "description": "Whether to perform case-sensitive matching",
                "default": false
            },
            "context_lines": {
                "type": "integer",
                "description": "Number of context lines to include around matches",
                "default": 2,
                "minimum": 0
            }
        },
        "required": ["session_id", "pattern"]
    })
}

pub async fn grep_file(
    mcp: &Mcp, 
    session_id: &str, 
    path: &str, 
    pattern: &str, 
    case_sensitive: bool, 
    context_lines: usize
) -> Result<Value> {
    match mcp.client.grep_file(session_id, PathBuf::from(path), pattern, case_sensitive, context_lines).await {
        Ok(file_matches) => {
            let mut content = vec![
                json!({
                    "type": "text",
                    "text": format!("Search results for pattern '{}' in {}", pattern, path)
                })
            ];

            if file_matches.is_empty() {
                content.push(json!({
                    "type": "text",
                    "text": "\nNo matches found"
                }));
            } else if let Some(file_match) = file_matches.first() {
                content.push(json!({
                    "type": "text", 
                    "text": format_matches(file_match)
                }));
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(format_error_response(e.to_string()))
    }
}

pub async fn grep_codebase(
    mcp: &Mcp, 
    session_id: &str, 
    path: Option<&str>, 
    file_pattern: Option<&str>, 
    pattern: &str,
    case_sensitive: bool, 
    context_lines: usize
) -> Result<Value> {
    let path = path.map(PathBuf::from);
    match mcp.client.grep_codebase(session_id, path.as_deref(), file_pattern, pattern, case_sensitive, context_lines).await {
        Ok(file_matches) => {
            let mut content = vec![];

            if file_matches.is_empty() {
                content.push(json!({
                    "type": "text",
                    "text": "\nNo matches found in any files"
                }));
            } else {
                for file_match in file_matches {
                    if !file_match.matches.is_empty() {
                        content.push(json!({
                            "type": "text",
                            "text": format_matches(&file_match)
                        }));
                    }
                }
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(format_error_response(e.to_string()))
    }
}

fn format_matches(file_match: &GrepFileMatch) -> String {
    let path_display = file_match.path.display();
    if file_match.matches.is_empty() {
        return format!("\nIn file {}: No matches found", path_display);
    }

    let matches_text = file_match.matches
        .iter()
        .map(|m| {
            format!("Match on line {}:\n{}", m.line_number, m.context)
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!("\nIn file {}:\n{}", path_display, matches_text)
}