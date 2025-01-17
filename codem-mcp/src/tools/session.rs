use serde_json::{json, Value};

pub fn tool_spec() -> Value {
    json!({
        "name": "create_session",
        "description": "Create a new Codem session for a project",
        "inputSchema": {
            "type": "object",
            "properties": {
                "project": {
                    "type": "string",
                    "description": "Name of the project to create a session for"
                }
            },
            "required": ["project"]
        }
    })
}