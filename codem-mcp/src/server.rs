use std::sync::Arc;
use std::path::PathBuf;
use codem_client::{Client, ClientConfig};
use jsonrpc_stdio_server::jsonrpc_core::{IoHandler, Params, Value, Result};
use serde::Deserialize;
use serde_json::json;

use crate::error::McpError;
use crate::tools;

/// MCP server state
pub struct MCP {
    client: Client,
}

impl MCP {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            client: Client::new(config)
        }
    }

    pub async fn create_session(&self, project: String) -> Result<Value> {
        self.client.create_session(&project)
            .await
            .map(|session_id| json!({
                "content": [{
                    "type": "text",
                    "text": json!({
                        "session_id": session_id
                    }).to_string()
                }]
            }))
            .map_err(|e| McpError::Client(e).into())
    }

    pub async fn read_file(&self, session_id: String, path: PathBuf) -> Result<Value> {
        self.client.read_file(&session_id, &path)
            .await
            .map(|content| json!({
                "content": [{
                    "type": "text",
                    "text": content
                }]
            }))
            .map_err(|e| McpError::Client(e).into())
    }
}

/// Create and run MCP server with given config
pub async fn serve(config: ClientConfig) -> Result<()> {
    let mcp = Arc::new(MCP::new(config)); 
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
        // No response needed for notifications
    });

    // Register tool listing
    io.add_method("tools/list", move |_params: Params| async move {
        Ok(tools::list_tools())
    });

    // Register tool calling
    let mcp_clone = mcp.clone();
    io.add_method("tools/call", move |params: Params| {
        let mcp = mcp_clone.clone();

        async move {
            #[derive(Deserialize)]
            struct ToolCall {
                name: String,
                arguments: serde_json::Value,
            }

            let call: ToolCall = params.parse()?;
            match call.name.as_str() {
                "create_session" => {
                    let project = call.arguments.get("project")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing project parameter"))?;
                    mcp.create_session(project.to_string()).await
                },
                "read_file" => {
                    let session_id = call.arguments.get("session_id")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing session_id parameter"))?;
                        
                    let path = call.arguments.get("path")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| jsonrpc_stdio_server::jsonrpc_core::Error::invalid_params("missing path parameter"))?;
                        
                    mcp.read_file(session_id.to_string(), PathBuf::from(path)).await
                },
                _ => Err(jsonrpc_stdio_server::jsonrpc_core::Error::method_not_found())
            }
        }
    });

    // Start server
    let server = jsonrpc_stdio_server::ServerBuilder::new(io)
        .build();

    server.await;
    Ok(())
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