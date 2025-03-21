use std::sync::Arc;
use codem_client::{Client, ClientConfig};
use jsonrpc_stdio_server::jsonrpc_core::{IoHandler, Params, Result};
use serde_json::json;
use tracing::info;

use crate::tools;

#[cfg(test)]
use {
    anyhow::Result as AnyhowResult,
    std::{fs, path::Path},
    tempfile::TempDir,
};

pub struct Mcp {
    pub(crate) client: Client,
}

impl Mcp {
    pub async fn new(config: ClientConfig) -> Self {
        Self {
            client: Client::new(config).await
        }
    }
}

/// Create and run MCP server with given config
pub async fn serve(config: ClientConfig) -> Result<()> {
    let mcp = Arc::new(Mcp::new(config).await); 
    let mut io = IoHandler::default();

    // Register initialization
    io.add_method("initialize", move |_params: Params| async move {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "serverInfo": {
                "name": "codem-mcp",
                "version": "0.1.0"
            },
            "capabilities": {
                "tools": {}
            }
        }))
    });

    // Register initialized notification
    io.add_notification("initialized", |_params: Params| {
        info!("initialized notification");
        // No response needed for notifications
    });

    // Register tool listing
    io.add_method("tools/list", move |_params: Params| async move {
        info!("LIST TOOLS");
        Ok(tools::list_tools())
    });

    // Register tool calling
    let mcp_clone = mcp.clone();
    io.add_method("tools/call", move |params: Params| {
        let mcp = mcp_clone.clone();

        async move {
            let call: tools::ToolCall = params.parse()?;
            tools::handle_tool_call(&mcp, call).await
        }
    });

    // Start server
    let server = jsonrpc_stdio_server::ServerBuilder::new(io)
        .build();

    server.await;
    Ok(())
}
