use jsonrpc_stdio_server::jsonrpc_core::{Error as RpcError, ErrorCode};
use serde_json::json;
use thiserror::Error;
use codem_client::error::ClientError;

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

pub fn error_response_with_content(err: &ClientError, content: Option<&str>) -> serde_json::Value {
    let message = err.to_string();
    
    let mut response = json!({
        "content": [{
            "type": "text",
            "text": message
        }],
        "isError": true
    });

    if let Some(content) = content {
        response.as_object_mut().unwrap()
            .get_mut("content").unwrap()
            .as_array_mut().unwrap()
            .push(json!({
                "type": "text",
                "text": format!("\nFile content:\n{}", content)
            }));
    }

    response
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

// Helper function to extract content from client errors
pub fn get_error_content(err: &ClientError) -> Option<&str> {
    match err {
        ClientError::WriteError(write_err) => match write_err {
            codem_core::error::WriteError::TimestampMismatch { content } => Some(content),
            codem_core::error::WriteError::MultiplePatternMatches { content, .. } => Some(content),
            codem_core::error::WriteError::StartPatternNotFound { content } => Some(content),
            codem_core::error::WriteError::EndPatternNotFound { content } => Some(content),
            codem_core::error::WriteError::MultipleStartPatternsFound { content } => Some(content),
            codem_core::error::WriteError::MultipleEndPatternsFound { content } => Some(content),
            codem_core::error::WriteError::EndPatternBeforeStart { content } => Some(content),
            codem_core::error::WriteError::InvalidPatternPair { content } => Some(content),
            _ => None,
        },
        ClientError::FileNotSynced { content } => content.as_deref(),
        ClientError::FileModifiedSinceRead { content } => content.as_deref(),
        ClientError::TimestampMismatch { content, .. } => Some(content),
        ClientError::FileNotReadable { content, .. } => content.as_deref(),
        ClientError::FileNotRead { content, .. } => content.as_deref(),
        ClientError::PathExists { content, .. } => content.as_deref(),
        ClientError::PermissionDenied { content, .. } => content.as_deref(),
        _ => None,
    }
}
