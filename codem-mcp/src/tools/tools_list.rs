use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::Value;
use crate::tools::{session, read, list, grep, write};

pub fn list_tools() -> Value {
    json!({
        "tools": [
            {
                "name": "create_session",
                "description": "Create a new Codem session for a project, as well as run a test command, grep the codebase, or list a directory. The session ID returned from this tool is needed for all other tools.",
                "inputSchema": session::create_session_schema()
            },
            {
                "name": "read_files",
                "description": "Read the contents of one or more files. To reduce the number of times read_files is called, if you have to read many files at once, you should provide all of them to one read_files call, rather than making multiple read_files calls.",
                "inputSchema": read::read_files_schema()
            },
            {
                "name": "list_directory",
                "description": "List contents of a directory with optional path regex filtering",
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
                "name": "create_new_file",
                "description": "Create a new file with the specified content",
                "inputSchema": write::write_new_file_schema()
            },
            {
                "name": "write_file_full",
                "description": "Write complete new content to a file. This tool is expensive! Try to use write_file_small or write_file_large, and only use write_file_full if you need to fully rewrite the file or one of small/large fails.",
                "inputSchema": write::write_file_full_schema()
            },
            {
                "name": "write_file_small", 
                "description": "Make one or more small text changes in a file. For each change, the old_str argument must be unique within the line range provided, unless you set allow_multiple_matches to true.",
                "inputSchema": write::write_file_small_schema()
            },
            {
                "name": "write_file_large",
                "description": "Replace a large section of text between start and end markers. Useful for replacing blocks of text. The markers must be unique in the target range, cannot overlap/contain each other, and cannot appear multiple times in the range. More reliable than write_file_full for partial changes that are too large for write_file_small.",
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
                "description": "Run the test command configured for the project. You should always test after you finish making a change to the codebase.",
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