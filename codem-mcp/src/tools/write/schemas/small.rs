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
            "changes": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "old_str": {"type": "string"},
                        "new_str": {"type": "string"}
                    },
                    "required": ["old_str", "new_str"]
                },
                "description": "List of changes to make - each specifies old text to find and new text to replace it with"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run tests after write",
                "default": true
            }
        },
        "required": ["session_id", "path", "changes"]
    })
}