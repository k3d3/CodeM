pub mod session;

use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::Value;

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
                "inputSchema": session::read_file_schema()
            }
        ]
    })
}