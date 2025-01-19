use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::Value;

pub fn schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for writing"
            },
            "path": {
                "type": "string",
                "description": "Path of new file to create (relative to project root)"
            },
            "content": {
                "type": "string",
                "description": "Content to write to new file"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run tests after write",
                "default": true
            }
        },
        "required": ["session_id", "path", "content"]
    })
}
