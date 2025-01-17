use thiserror::Error;
use jsonrpc_stdio_server::jsonrpc_core::{Error as RpcError, ErrorCode};

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Client error: {0}")]
    Client(#[from] codem_client::ClientError)
}

impl From<McpError> for RpcError {
    fn from(err: McpError) -> Self {
        match err {
            McpError::Client(e) => {
                let mut error = RpcError::new(ErrorCode::ServerError(-32000));
                error.message = e.to_string();
                error
            }
        }
    }
}