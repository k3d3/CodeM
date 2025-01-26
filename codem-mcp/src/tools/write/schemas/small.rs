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
            "old_str": {
                "type": "string",
                "description": "Text to find and replace - must be unique in the specified line range unless allow_multiple_matches is true. Leading/trailing whitespace is ignored in matching. The text does not need to match entire lines. Keep under a few lines - for larger changes use write_file_large."
            },
            "new_str": {
                "type": "string",
                "description": "Text to replace it with"
            },
            "allow_multiple_matches": {
                "type": "boolean",
                "description": "Whether to allow replacing multiple matches of old_str. By default this is false to prevent unintended changes.",
                "default": false
            },
            "line_range": {
                "type": "object",
                "properties": {
                    "start": {
                        "type": ["integer", "null"],
                        "description": "Start of search range for finding old_str (1-based, inclusive). Can be approximate - the range just needs to contain exactly one instance of old_str. If null, starts from beginning."
                    },
                    "end": {
                        "type": ["integer", "null"],
                        "description": "End of search range for finding old_str (1-based, inclusive). Can be approximate - the range just needs to contain exactly one instance of old_str. If null, continues to end."
                    }
                }
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
