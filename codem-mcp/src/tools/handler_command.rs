use std::path::Path;
use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Value, Result, Error};
use crate::{server::Mcp, tools::types::ToolCall};

pub async fn handle_run_command(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let command = call.arguments.get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing command parameter"))?;

    let cwd = call.arguments.get("cwd")
        .and_then(|v| v.as_str())
        .map(Path::new);

    let timeout = call.arguments.get("timeout")
        .and_then(|v| v.as_u64());

    match mcp.client.run_command(session_id, command, cwd, timeout).await {
        Ok(output) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": output
                }
            ]
        })),
        Err(err) => {
            if matches!(err, codem_client::error::ClientError::UnsafeCommand { .. }) {
                Ok(json!({
                    "content": [
                        {
                            "type": "text", 
                            "text": format!("Command '{}' is not marked as safe. Use run_command_risky if you want to run this command.", command)
                        }
                    ]
                }))
            } else {
                Ok(json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Command failed: {}", err)
                        }
                    ]
                }))
            }
        }
    }
}

pub async fn handle_run_command_risky(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let command = call.arguments.get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing command parameter"))?;

    let cwd = call.arguments.get("cwd")
        .and_then(|v| v.as_str())
        .map(Path::new);

    let timeout = call.arguments.get("timeout")
        .and_then(|v| v.as_u64());

    match mcp.client.run_command_risky(session_id, command, cwd, timeout).await {
        Ok(output) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": output
                }
            ]
        })),
        Err(err) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": format!("Command failed: {}", err)
                }
            ]
        }))
    }
}

pub async fn handle_run_test_command(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    match mcp.client.run_test_command(session_id).await {
        Ok(output) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": output
                }
            ]
        })),
        Err(err) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": format!("Test command failed: {}", err)
                }
            ]
        }))
    }
}