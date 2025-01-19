use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use crate::{server::Mcp, error::format_error_response};
use crate::tools::types::ToolCall;

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

pub async fn create_session(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let project = call.arguments.get("project")
        .and_then(|v| v.as_str())
        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing project parameter"))?;

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