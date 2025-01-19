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
                "description": "Path to file (relative to project root)"
            },
            "content": {
                "type": "string",
                "description": "New content to write to file"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run tests after write. Highly recommended if this is the last write operation in a series.",
                "default": false
            }
        },
        "required": ["session_id", "path", "content"]
    })
}