use std::sync::Arc;
use codem_client::{Client, ClientConfig};
use jsonrpc_stdio_server::jsonrpc_core::{IoHandler, Params, Value, Result};
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::error::McpError;
use crate::tools;

/// MCP server state
pub struct MCP {
    client: Client,
    runtime: Runtime,
}

impl MCP {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            client: Client::new(config),
            runtime: Runtime::new().unwrap()
        }
    }

    pub fn create_session(&self, project: String) -> Result<Value> {
        let fut = self.client.create_session(&project);
        // Block on the future since jsonrpc-core is synchronous
        self.runtime.block_on(fut)
            .map(Value::String)
            .map_err(|e| McpError::Client(e).into())
    }
}

/// Create and run MCP server with given config
pub fn serve(config: ClientConfig) -> Result<()> {
    let mcp = Arc::new(MCP::new(config)); 
    let mut io = IoHandler::default();

    // Register tool listing
    io.add_sync_method("tools/list", move |_params: Params| {
        Ok(tools::list_tools())
    });

    // Register tool calling
    let mcp_clone = mcp.clone();
    io.add_sync_method("tools/call", move |params: Params| {
        #[derive(Deserialize)]
        struct ToolCall {
            name: String,
            arguments: serde_json::Value,
        }

        let call: ToolCall = params.parse()?;
        tools::handle_call(&call.name, &mcp_clone, &call.arguments)
    });

    // Start server
    let runtime = Runtime::new().unwrap();
    let server = jsonrpc_stdio_server::ServerBuilder::new(io)
        .build();

    // Use block_on instead of wait() since we're using tokio runtime
    runtime.block_on(async {
        server.await;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use codem_client::Project;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_new() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let mut project = Project::new(temp_path.to_path_buf());
        project.allowed_paths = Some(vec![temp_path.to_path_buf()]);
        let projects = vec![project];

        let session_dir = temp_path.join("session");
        fs::create_dir_all(&session_dir).unwrap();
        fs::write(
            session_dir.join("session.toml"),
            "# Codem session file\n"
        ).unwrap();

        let config = ClientConfig::new(
            projects,
            temp_path.join("session").join("session.toml"),
            vec!["^echo [a-zA-Z0-9_-]+$".to_string()],
            vec![]
        ).unwrap();

        let _mcp = MCP::new(config);
    }
}