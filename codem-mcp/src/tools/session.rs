use serde_json::json;

pub fn create_session_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "project": {
                "type": "string",
                "description": "Project name to create session for"
            }
        },
        "required": ["project"]
    })
}

pub fn read_file_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for reading"
            },
            "path": {
                "type": "string",
                "description": "Path to file (relative to project root)"
            }
        },
        "required": ["session_id", "path"]
    })
}