pub mod session;

use serde_json::{json, Value};
use jsonrpc_stdio_server::jsonrpc_core::{Result, Error as RpcError};
use crate::server::MCP;

/// Handle a tool call by dispatching to the appropriate handler
pub fn handle_call(name: &str, mcp: &MCP, args: &Value) -> Result<Value> {
    match name {
        "create_session" => session::handle_call(mcp, args),
        _ => Err(RpcError::method_not_found())
    }
}

/// Get all available tools
pub fn list_tools() -> Value {
    let tools = vec![
        session::tool_spec(),
        // Add more tools here
    ];
    json!({ "tools": tools })
}