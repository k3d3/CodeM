use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use std::path::PathBuf;
use crate::server::Mcp;

pub fn read_files_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for reading"
            },
            "paths": {
                "type": "array",
                "items": {
                    "type": "string",
                    "description": "Path to file (relative to project root)"
                },
                "description": "Array of file paths to read"
            }
        },
        "required": ["session_id", "paths"]
    })
}

pub async fn read_files(mcp: &Mcp, session_id: &str, paths: Vec<String>) -> Result<Value> {
    let mut files_text = String::new();
    let mut successful_reads = 0;
    let mut failed_reads = 0;
    let total_files = paths.len();

    if paths.is_empty() {
        files_text.push_str("No files provided.");
    } else {
        for path in paths {
            match mcp.client.read_file(session_id, &PathBuf::from(&path)).await {
                Ok((content, metadata)) => {
                    successful_reads += 1;
                    files_text.push_str(&format!("\nFile: {}\n", path));
                    files_text.push_str(&format!("Size: {} bytes\n", metadata.size.unwrap_or(0)));
                    files_text.push_str(&format!("Lines: {}\n", metadata.line_count.unwrap_or(0)));
                    files_text.push_str("Content:\n");
                    files_text.push_str(&content);
                    files_text.push_str("\n---\n");
                },
                Err(e) => {
                    failed_reads += 1;
                    files_text.push_str(&format!("\nFile: {}\n", path));
                    files_text.push_str(&format!("Error: {}\n", e));
                    files_text.push_str("---\n");
                }
            }
        }
    }

    let summary = format!(
        "Summary:\nTotal files: {}\nSuccessful reads: {}\nFailed reads: {}", 
        total_files, successful_reads, failed_reads
    );

    Ok(json!({
        "content": [
            {
                "type": "text",
                "text": files_text
            },
            {
                "type": "text",
                "text": summary
            }
        ]
    }))
}