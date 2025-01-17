use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value};
use std::path::PathBuf;
use crate::server::Mcp;
use codem_core::types::ListOptions;

pub fn list_directory_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {
                "type": "string",
                "description": "Session ID to use for reading"
            },
            "path": {
                "type": "string",
                "description": "Path to directory (relative to project root)",
                "optional": true
            },
            "recursive": {
                "type": "boolean",
                "description": "Whether to list directories recursively",
                "optional": true
            },
            "count_lines": {
                "type": "boolean",
                "description": "Whether to count lines in files",
                "optional": true
            }, 
            "include_size": {
                "type": "boolean",
                "description": "Whether to include file sizes",
                "optional": true
            },
            "file_pattern": {
                "type": "string",
                "description": "Optional regex to filter filenames",
                "optional": true
            }
        },
        "required": ["session_id"]
    })
}

fn format_tree_entry(entry: &codem_core::types::TreeEntry, include_stats: bool) -> String {
    let mut output = String::new();
    let path = entry.path().file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| entry.path().to_string_lossy().to_string());

    let entry_type = if entry.is_dir() { "📁" } else { "📄" };

    // Add stats info if requested and available
    let stats = if include_stats {
        if let Some(stats) = entry.stats() {
            let mut parts = Vec::new();
            if let Some(lines) = stats.line_count {
                parts.push(format!("{} lines", lines));
            }
            if let Some(size) = stats.size {
                parts.push(format!("{} bytes", size));
            }
            if !parts.is_empty() {
                format!(" ({})", parts.join(", "))
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    output.push_str(&format!("{} {}{}", entry_type, path, stats));

    // Add children if it's a directory and has children
    if entry.is_dir() && !entry.is_empty() {
        for (i, child) in entry.iter().enumerate() {
            let is_last = i == entry.len() - 1;
            let prefix = if is_last { "└─" } else { "├─" };
            
            // Format child entry
            let child_str = format_tree_entry(child, include_stats);
            
            // Split child lines and add proper prefixes
            for (j, line) in child_str.lines().enumerate() {
                if j == 0 {
                    output.push_str(&format!("\n{}{}", prefix, line));
                } else {
                    let cont_prefix = if is_last { "  " } else { "│ " };
                    output.push_str(&format!("\n{}{}", cont_prefix, line));
                }
            }
        }
    }

    output
}

pub async fn list_directory(mcp: &Mcp, session_id: &str, path: Option<&str>, options: ListOptions) -> Result<Value> {
    let path = path.map(PathBuf::from);
    
    let include_stats = options.count_lines || options.include_size;
    let result = mcp.client.list_directory(session_id, path.as_deref(), options.clone())
        .await
        .map_err(|e| jsonrpc_stdio_server::jsonrpc_core::Error::from(crate::error::McpError::Client(e)))?;

    let formatted = format_tree_entry(&result, include_stats);

    Ok(json!({
        "content": [
            {
                "type": "text",
                "text": formatted
            }
        ]
    }))
}