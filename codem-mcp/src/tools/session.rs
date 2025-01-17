use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use crate::server::Mcp;

pub fn create_session_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "project": {
                "type": "string",
                "description": "Project name to create session for"
            }
        },
        "required": ["project"]
    })
}

pub async fn create_session(mcp: &Mcp, project: &str) -> Result<Value> {
    mcp.client.create_session(project)
        .await
        .map(|session_id| json!({
            "content": [{
                "type": "text",
                "text": json!({
                    "session_id": session_id
                }).to_string()
            }]
        }))
        .map_err(|e| crate::error::McpError::Client(e).into())
}