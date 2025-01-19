mod schemas;

use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value}; 
use std::path::PathBuf;
use crate::{server::Mcp, error::{error_response_with_content, get_error_content}};

pub use schemas::*;

pub async fn write_file_full(mcp: &Mcp, session_id: &str, path: &str, content: &str, run_test: bool) -> Result<Value> {
    match mcp.client.write_file_full(session_id, &PathBuf::from(path), content, run_test).await {
        Ok(result) => {
            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "File written successfully:\nPath: {}\nSize: {}\nLines: {}", 
                        path,
                        result.size,
                        result.line_count,
                    )
                }]
            }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}

pub async fn write_file_small(mcp: &Mcp, session_id: &str, path: &str, changes: Vec<codem_core::types::Change>, run_test: bool) -> Result<Value> {
    match mcp.client.write_file_partial(session_id, &PathBuf::from(path), changes, run_test).await {
        Ok(result) => {
            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "File updated successfully:\nPath: {}\nSize: {}\nLines: {}",
                        path,
                        result.size,
                        result.line_count,
                    )
                }]
            }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}

pub async fn write_file_large(mcp: &Mcp, session_id: &str, path: &str, start_str: &str, end_str: &str, new_str: &str, run_test: bool) -> Result<Value> {
    match mcp.client.write_file_large(session_id, &PathBuf::from(path), start_str, end_str, new_str, run_test).await {
        Ok(result) => {
            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "File section replaced successfully:\nPath: {}\nSize: {}\nLines: {}",
                        path,
                        result.size,
                        result.line_count,
                    )
                }]
            }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}