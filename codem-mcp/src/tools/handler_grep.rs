use jsonrpc_stdio_server::jsonrpc_core::{Value, Result, Error};
use crate::{server::Mcp, tools::grep};
use crate::tools::types::ToolCall;

pub async fn handle_grep_file(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let path = call.arguments.get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing path parameter"))?;

    let pattern = call.arguments.get("pattern")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing pattern parameter"))?;

    let case_sensitive = call.arguments.get("case_sensitive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let context_lines = call.arguments.get("context_lines")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    grep::grep_file(mcp, session_id, path, pattern, case_sensitive, context_lines).await
}

pub async fn handle_grep_codebase(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let pattern = call.arguments.get("pattern")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing pattern parameter"))?;

    let path = call.arguments.get("path")
        .and_then(|v| v.as_str());

    let file_pattern = call.arguments.get("file_pattern")
        .and_then(|v| v.as_str());

    let case_sensitive = call.arguments.get("case_sensitive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let context_lines = call.arguments.get("context_lines")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    grep::grep_codebase(mcp, session_id, path, file_pattern, pattern, case_sensitive, context_lines).await
}