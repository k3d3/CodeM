use jsonrpc_stdio_server::jsonrpc_core::{Error as RpcError, ErrorCode};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)] 
pub enum McpError {
    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl From<McpError> for RpcError {
    fn from(err: McpError) -> Self {
        match err {
            McpError::Protocol(msg) => RpcError {
                code: ErrorCode::MethodNotFound,
                message: msg,
                data: None
            },
            McpError::Internal(msg) => RpcError {
                code: ErrorCode::InternalError,
                message: msg,
                data: None
            }
        }
    }
}

pub fn format_error_response(message: impl Into<String>) -> serde_json::Value {
    json!({
        "content": [{
            "type": "text",
            "text": message.into()
        }],
        "isError": true
    })
}