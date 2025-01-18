pub mod session;
pub mod list;
pub mod read;
pub mod grep;
pub mod write;

use serde_json::json;
use serde::Deserialize;
use jsonrpc_stdio_server::jsonrpc_core::{Value, Result};
use crate::server::Mcp;
use codem_core::types::{ListOptions, Change};

#[derive(Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

pub fn list_tools() -> Value {
    json!({
        "tools": [
            {
                "name": "create_session",
                "description": "Create a new Codem session for a project. This is needed for all other commands.",
                "inputSchema": session::create_session_schema()
            },
            {
                "name": "read_files",
                "description": "Read the contents of one or more files",
                "inputSchema": read::read_files_schema()
            },
            {
                "name": "list_directory",
                "description": "List contents of a directory with optional regex filtering",
                "inputSchema": list::list_directory_schema()
            },
            {
                "name": "grep_file",
                "description": "Search for a pattern in a specific file",
                "inputSchema": grep::grep_file_schema()
            },
            {
                "name": "grep_codebase",
                "description": "Search for a pattern across multiple files",
                "inputSchema": grep::grep_codebase_schema()
            },
            {
                "name": "write_file_full",
                "description": "Write complete new content to a file",
                "inputSchema": write::write_file_full_schema()
            },
            {
                "name": "write_file_small", 
                "description": "Make small text replacements in a file",
                "inputSchema": write::write_file_small_schema()
            },
            {
                "name": "write_file_large",
                "description": "Replace a large section of text between start and end markers",
                "inputSchema": write::write_file_large_schema()
            }
        ]
    })
}

pub async fn handle_tool_call(mcp: &Mcp, call: ToolCall) -> Result<Value> {
    match call.name.as_str() {
        "create_session" => {
            let project = call.arguments.get("project")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing project parameter"))?;
            session::create_session(mcp, project).await
        },
        "read_files" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;
                
            let paths = call.arguments.get("paths")
                .and_then(|v| v.as_array())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing paths parameter"))?;

            let paths: Vec<String> = paths.iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect();
                
            read::read_files(mcp, session_id, paths).await
        },
        "list_directory" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;
                
            let path = call.arguments.get("path")
                .and_then(|v| v.as_str());

            let options = ListOptions {
                recursive: call.arguments.get("recursive").and_then(|v| v.as_bool()).unwrap_or(false),
                count_lines: call.arguments.get("count_lines").and_then(|v| v.as_bool()).unwrap_or(false),
                include_size: call.arguments.get("include_size").and_then(|v| v.as_bool()).unwrap_or(false),
                file_pattern: call.arguments.get("file_pattern").and_then(|v| v.as_str()).map(String::from),
                include_modified: false,
            };
                
            list::list_directory(mcp, session_id, path, options).await
        },
        "grep_file" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;

            let path = call.arguments.get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing path parameter"))?;

            let pattern = call.arguments.get("pattern")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing pattern parameter"))?;

            let case_sensitive = call.arguments.get("case_sensitive")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let context_lines = call.arguments.get("context_lines")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            grep::grep_file(mcp, session_id, path, pattern, case_sensitive, context_lines).await
        },
        "grep_codebase" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;

            let pattern = call.arguments.get("pattern")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing pattern parameter"))?;

            let path = call.arguments.get("path")
                .and_then(|v| v.as_str());

            let file_pattern = call.arguments.get("file_pattern")
                .and_then(|v| v.as_str());

            let case_sensitive = call.arguments.get("case_sensitive")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let context_lines = call.arguments.get("context_lines")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            grep::grep_codebase(mcp, session_id, path, file_pattern, pattern, case_sensitive, context_lines).await
        },
        "write_file_full" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;

            let path = call.arguments.get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing path parameter"))?;

            let content = call.arguments.get("content")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing content parameter"))?;

            let run_test = call.arguments.get("run_test")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            write::write_file_full(mcp, session_id, path, content, run_test).await
        },
        "write_file_small" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;

            let path = call.arguments.get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing path parameter"))?;

            let changes = call.arguments.get("changes")
                .and_then(|v| v.as_array())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing changes parameter"))?;

            let changes: Result<Vec<Change>> = changes.iter()
                .map(|v| {
                    let old_str = v.get("old_str")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing old_str in change"))?;
                    
                    let new_str = v.get("new_str")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing new_str in change"))?;

                    Ok(Change {
                        old_str: old_str.to_string(),
                        new_str: new_str.to_string(),
                        allow_multiple_matches: false,
                    })
                })
                .collect();

            let changes = changes?;
            let run_test = call.arguments.get("run_test")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            write::write_file_small(mcp, session_id, path, changes, run_test).await
        },
        "write_file_large" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;

            let path = call.arguments.get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing path parameter"))?;

            let start_str = call.arguments.get("start_str")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing start_str parameter"))?;

            let end_str = call.arguments.get("end_str")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing end_str parameter"))?;

            let new_str = call.arguments.get("new_str")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing new_str parameter"))?;

            let run_test = call.arguments.get("run_test")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            write::write_file_large(mcp, session_id, path, start_str, end_str, new_str, run_test).await
        },
        _ => Ok(crate::error::format_error_response(format!("Unknown tool: {}", call.name)))
    }
}