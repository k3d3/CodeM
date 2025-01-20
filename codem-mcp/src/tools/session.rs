use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use codem_core::types::{ListOptions, GrepOptions};
use crate::{server::Mcp, error::format_error_response};
use crate::tools::types::ToolCall;

pub fn create_session_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "project": {
                "type": "string",
                "description": "Project name to create session for"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run the test command after session creation",
                "default": false
            },
            "list_directory": {
                "type": "object",
                "description": "List directory after session creation",
                "properties": {
                    "recursive": {
                        "type": "boolean",
                        "description": "Whether to list directories recursively",
                        "default": false
                    },
                    "file_pattern": {
                        "type": "string",
                        "description": "Optional regex to filter filenames"
                    }
                }
            },
            "grep_pattern": {
                "type": "object",
                "description": "Pattern and options for searching codebase after session creation",
                "properties": {
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
                        "description": "Number of context lines around matches",
                        "default": 2,
                        "minimum": 0
                    },
                    "file_pattern": {
                        "type": "string",
                        "description": "Optional regex to filter files to search"
                    }
                },
                "required": ["pattern"]
            }
        },
        "required": ["project"]
    })
}

pub async fn create_session(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let project = call.arguments.get("project")
        .and_then(|v| v.as_str())
        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing project parameter"))?;

    let session_result = match mcp.client.create_session(project).await {
        Ok(session_id) => {
            let mut content = Vec::new();
            
            // Add session ID to content
            content.push(json!({
                "type": "text",
                "text": json!({
                    "session_id": session_id
                }).to_string()
            }));

            // Run test command if requested
            if let Some(run_test) = call.arguments.get("run_test").and_then(|v| v.as_bool()) {
                if run_test {
                    if let Ok(test_result) = mcp.client.run_test_command(&session_id).await {
                        content.push(json!({
                            "type": "text",
                            "text": format!("Test command result: {}", test_result)
                        }));
                    }
                }
            }

            // List directory if requested
            if let Some(list_dir) = call.arguments.get("list_directory") {
                if let Some(list_obj) = list_dir.as_object() {
                    let recursive = list_obj.get("recursive")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let file_pattern = list_obj.get("file_pattern")
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    let options = ListOptions {
                        recursive,
                        file_pattern,
                        ..Default::default()
                    };

                    if let Ok(list_result) = mcp.client.list_directory(
                        &session_id,
                        None,
                        options,
                    ).await {
                        content.push(json!({
                            "type": "text",
                            "text": format!("Directory listing:\n{:#?}", list_result)
                        }));
                    }
                }
            }

            // Grep codebase if requested
            if let Some(grep_obj) = call.arguments.get("grep_pattern").and_then(|v| v.as_object()) {
                let pattern = grep_obj.get("pattern")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing grep pattern"))?
                    .to_string();
                    
                let options = GrepOptions {
                    case_sensitive: grep_obj.get("case_sensitive")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    context_lines: grep_obj.get("context_lines")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as usize)
                        .unwrap_or(2),
                    file_pattern: grep_obj.get("file_pattern")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                };

                if let Ok(grep_result) = mcp.client.grep_codebase(
                    &session_id,
                    None,
                    None,
                    &pattern,
                    options.case_sensitive,
                    options.context_lines
                ).await {
                    content.push(json!({
                        "type": "text",
                        "text": format!("Grep results:\n{:#?}", grep_result)
                    }));
                }
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(format_error_response(e.to_string()))
    };

    session_result
}