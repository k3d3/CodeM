use jsonrpc_stdio_server::jsonrpc_core::{Value, Result, Error};
use crate::{server::Mcp, tools::write};
use crate::tools::types::ToolCall;

pub async fn handle_write_new_file(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let path = call.arguments.get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing path parameter"))?;

    let content = call.arguments.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing content parameter"))?;

    let run_test = call.arguments.get("run_test")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    write::write_new_file(mcp, session_id, path, content, run_test).await
}

pub async fn handle_write_file_full(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let path = call.arguments.get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing path parameter"))?;

    let content = call.arguments.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing content parameter"))?;

    let run_test = call.arguments.get("run_test")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    write::write_file_full(mcp, session_id, path, content, run_test).await
}

pub async fn handle_write_file_large(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let path = call.arguments.get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing path parameter"))?;

    let start_str = call.arguments.get("start_str")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing start_str parameter"))?;

    let end_str = call.arguments.get("end_str")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing end_str parameter"))?;

    let new_str = call.arguments.get("new_str")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing new_str parameter"))?;

    let run_test = call.arguments.get("run_test")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    write::write_file_large(mcp, session_id, path, start_str, end_str, new_str, run_test).await
}