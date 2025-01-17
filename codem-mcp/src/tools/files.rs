use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use std::path::PathBuf;
use crate::server::Mcp;

pub fn read_file_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for reading"
            },
            "path": {
                "type": "string",
                "description": "Path to file (relative to project root)"
            }
        },
        "required": ["session_id", "path"]
    })
}

pub async fn read_file(mcp: &Mcp, session_id: &str, path: &str) -> Result<Value> {
    mcp.client.read_file(session_id, &PathBuf::from(path))
        .await
        .map(|(content, metadata)| json!({
            "content": [
                {
                    "type": "text",
                    "text": content
                },
                {
                    "type": "text",
                    "text": format!(
                        "[METADATA]\nSIZE_BYTES={}\nLINE_COUNT={}\n[/METADATA]",
                        metadata.size.unwrap_or(0),
                        metadata.line_count.unwrap_or(0)
                    )
                }
            ]
        }))
        .map_err(|e| crate::error::McpError::Client(e).into())
}