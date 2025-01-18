use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::Value;
use crate::tools::{session, read, list, grep, write};

pub fn list_tools() -> Value {
    json!({
        "tools": [
            {
                "name": "create_session",
                "description": "Create a new Codem session for a project. This is needed for all other commands.",
                "inputSchema": session::create_session_schema()
            },
            {
                "name": "read_files",
                "description": "Read the contents of one or more files",
                "inputSchema": read::read_files_schema()
            },
            {
                "name": "list_directory",
                "description": "List contents of a directory with optional regex filtering",
                "inputSchema": list::list_directory_schema()
            },
            {
                "name": "grep_file",
                "description": "Search for a pattern in a specific file",
                "inputSchema": grep::grep_file_schema()
            },
            {
                "name": "grep_codebase",
                "description": "Search for a pattern across multiple files",
                "inputSchema": grep::grep_codebase_schema()
            },
            {
                "name": "write_file_full",
                "description": "Write complete new content to a file",
                "inputSchema": write::write_file_full_schema()
            },
            {
                "name": "write_file_small", 
                "description": "Make small text replacements in a file",
                "inputSchema": write::write_file_small_schema()
            },
            {
                "name": "write_file_large",
                "description": "Replace a large section of text between start and end markers",
                "inputSchema": write::write_file_large_schema()
            },
            {
                "name": "run_command",
                "description": "Run a safe command in the project directory",
                "inputSchema": command_schema()
            },
            {
                "name": "run_command_risky",
                "description": "Run a potentially unsafe command in the project directory",
                "inputSchema": command_schema()
            },
            {
                "name": "run_test_command",
                "description": "Run the test command configured for the project",
                "inputSchema": session_only_schema()
            }
        ]
    })
}

fn command_schema() -> Value {
    json!({
        "type": "object",
        "required": ["session_id", "command"],
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID for the project"
            },
            "command": {
                "type": "string",
                "description": "Command to execute"  
            },
            "cwd": {
                "type": "string",
                "description": "Working directory for command execution (optional)"
            },
            "timeout": {
                "type": "integer",
                "description": "Command timeout in seconds (optional)"
            }
        }
    })
}

fn session_only_schema() -> Value {
    json!({
        "type": "object",
        "required": ["session_id"],
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID for the project"
            }
        }
    })
}