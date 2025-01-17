pub mod session;
pub mod files;

use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Value, Result};
use crate::server::Mcp;
use serde::Deserialize;

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
                "description": "Create a new Codem session for a project",
                "inputSchema": session::create_session_schema()
            },
            {
                "name": "read_file",
                "description": "Read a file's contents", 
                "inputSchema": files::read_file_schema()
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
                
            files::read_file(mcp, session_id, path).await
        },
        _ => Err(jsonrpc_stdio_server::jsonrpc_core::Error::method_not_found())
    }
}