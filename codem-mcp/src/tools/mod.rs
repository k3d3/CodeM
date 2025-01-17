pub mod session;

use serde_json::{json, Value};

/// Get all available tools
pub fn list_tools() -> Value {
    let tools = vec![
        session::tool_spec(),
        // Add more tools here
    ];
    json!({ "tools": tools })
}