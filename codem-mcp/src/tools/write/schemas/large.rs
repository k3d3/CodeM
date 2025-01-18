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
            "start_str": {
                "type": "string",
                "description": "Start of text to replace"
            },
            "end_str": {
                "type": "string", 
                "description": "End of text to replace"
            },
            "new_str": {
                "type": "string",
                "description": "New text to insert between start and end"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run tests after write",
                "default": true
            }
        },
        "required": ["session_id", "path", "start_str", "end_str", "new_str"]
    })
}