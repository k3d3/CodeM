pub mod session;
pub mod list;
pub mod read;
pub mod grep;

use serde_json::json;
use serde::Deserialize;
use jsonrpc_stdio_server::jsonrpc_core::{Value, Result};
use crate::server::Mcp;
use codem_core::types::ListOptions;

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
                "name": "read_file",
                "description": "Read a file's contents", 
                "inputSchema": read::read_file_schema()
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
        "read_file" => {
            let session_id = call.arguments.get("session_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;
                
            let path = call.arguments.get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing path parameter"))?;
                
            read::read_file(mcp, session_id, path).await
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
        _ => Ok(crate::error::format_error_response(format!("Unknown tool: {}", call.name)))
    }
}