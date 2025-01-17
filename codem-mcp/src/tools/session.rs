use serde_json::{json, Value};
use jsonrpc_stdio_server::jsonrpc_core::{Error as RpcError, Result};
use crate::server::MCP;

pub fn tool_spec() -> Value {
    json!({
        "name": "create_session",
        "description": "Create a new Codem session for a project",
        "inputSchema": {
            "type": "object",
            "properties": {
                "project": {
                    "type": "string",
                    "description": "Name of the project to create a session for"
                }
            },
            "required": ["project"]
        }
    })
}

pub fn handle_call(mcp: &MCP, args: &Value) -> Result<Value> {
    let project = args.get("project")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RpcError::invalid_params("missing project parameter"))?;
        
    let session = mcp.create_session(project.to_string())?;
    Ok(session)
}