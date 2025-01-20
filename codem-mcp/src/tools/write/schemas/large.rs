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
            "line_range": {
                "type": "object",
                "properties": {
                    "start": {
                        "type": ["integer", "null"],
                        "description": "Starting line number (1-based, inclusive) to limit search range for start_str and end_str. If null, starts from beginning."
                    },
                    "end": {
                        "type": ["integer", "null"],
                        "description": "Ending line number (1-based, inclusive) to limit search range for start_str and end_str. If null, continues to end."
                    }
                },
                "description": "Optional line range to limit where matches can occur"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run tests after write. Highly recommended if this is the last write operation in a series.",
                "default": false
            }
        },
        "required": ["session_id", "path", "start_str", "end_str", "new_str"]
    })
}