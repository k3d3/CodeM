use jsonrpc_stdio_server::jsonrpc_core::{Value, Result, Error};
use crate::{server::Mcp, tools::{read, list}};
use crate::tools::types::ToolCall;
use codem_core::types::ListOptions;

pub async fn handle_read_files(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;
            
    let paths = call.arguments.get("paths")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::invalid_params("missing paths parameter"))?;

    let paths: Vec<String> = paths.iter()
        .filter_map(|v| v.as_str())
        .map(String::from)
        .collect();
            
    read::read_files(mcp, session_id, paths).await
}

pub async fn handle_list_directory(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;
            
    let path = call.arguments.get("path")
        .and_then(|v| v.as_str());

    let options = ListOptions {
        recursive: call.arguments.get("recursive").and_then(|v| v.as_bool()).unwrap_or(false),
        count_lines: call.arguments.get("count_lines").and_then(|v| v.as_bool()).unwrap_or(false),
        include_size: call.arguments.get("include_size").and_then(|v| v.as_bool()).unwrap_or(false),
        file_pattern: call.arguments.get("file_pattern").and_then(|v| v.as_str()).map(String::from),
        include_modified: false,
    };
            
    list::list_directory(mcp, session_id, path, options).await
}