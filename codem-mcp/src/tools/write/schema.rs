use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::Value;

pub fn write_file_full_schema() -> Value {
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
                "description": "Whether to run tests after write",
                "default": true
            }
        },
        "required": ["session_id", "path", "content"]
    })
}

pub fn write_file_small_schema() -> Value {
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

pub fn write_new_file_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for writing"
            },
            "path": {
                "type": "string",
                "description": "Path to new file to create (relative to project root)"
            },
            "content": {
                "type": "string",
                "description": "Content to write to the new file"
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

pub fn write_file_large_schema() -> Value {
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