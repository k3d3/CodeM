use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use std::path::PathBuf;
use crate::{server::Mcp, error::format_error_response};
use codem_core::types::ListOptions;
use super::format::format_tree_entry;

pub fn list_directory_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for reading"
            },
            "path": {
                "type": "string",
                "description": "Path to directory (relative to project root)",
                "optional": true
            },
            "recursive": {
                "type": "boolean",
                "description": "Whether to list directories recursively",
                "optional": true
            },
            "count_lines": {
                "type": "boolean",
                "description": "Whether to count lines in files",
                "optional": true
            }, 
            "include_size": {
                "type": "boolean",
                "description": "Whether to include file sizes",
                "optional": true
            },
            "file_pattern": {
                "type": "string",
                "description": "Optional regex to filter filenames (note that this does not imply recursive; you still need to set recursive to true)",
                "optional": true
            }
        },
        "required": ["session_id"]
    })
}

pub async fn list_directory(mcp: &Mcp, session_id: &str, path: Option<&str>, options: ListOptions) -> Result<Value> {
    let path = path.map(PathBuf::from);
    let include_stats = options.count_lines || options.include_size;
    
    match mcp.client.list_directory(session_id, path.as_deref(), options.clone()).await {
        Ok(result) => {
            let formatted = format_tree_entry(&result, include_stats);
            if formatted.is_empty() {
                Ok(format_error_response("Directory is empty".to_string()))
            } else {
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": formatted
                    }]
                }))
            }
        },
        Err(e) => Ok(format_error_response(format!("Failed to list directory: {}", e)))
    }
}