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
                        "old_str": {
                            "type": "string",
                            "description": "Text to find and replace"
                        },
                        "new_str": {
                            "type": "string",
                            "description": "Text to replace it with"
                        },
                        "allow_multiple_matches": {
                            "type": "boolean",
                            "description": "Whether to allow replacing multiple matches of old_str. By default this is false to prevent unintended changes.",
                            "default": false
                        }
                    },
                    "required": ["old_str", "new_str"]
                },
                "description": "List of changes to make - each specifies old text to find and new text to replace it with"
            },
            "run_test": {
                "type": "boolean",
                "description": "Whether to run tests after write. Highly recommended if this is the last write operation in a series.",
                "default": false
            }
        },
        "required": ["session_id", "path", "changes"]
    })
}
