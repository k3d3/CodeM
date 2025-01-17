use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use crate::{server::Mcp, error::format_error_response};

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
    match mcp.client.create_session(project).await {
        Ok(session_id) => Ok(json!({
            "content": [{
                "type": "text",
                "text": json!({
                    "session_id": session_id
                }).to_string()
            }]
        })),
        Err(e) => Ok(format_error_response(e.to_string()))
    }
}