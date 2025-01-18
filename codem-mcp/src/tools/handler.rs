use jsonrpc_stdio_server::jsonrpc_core::{Value, Result, Error};
use crate::server::Mcp;
use crate::tools::{
    session,
    handler_read,
    handler_write,
    handler_grep,
    handler_write_small,
    handler_command,
};
use crate::tools::types::ToolCall;

pub async fn handle_tool_call(mcp: &Mcp, call: ToolCall) -> Result<Value> {
    match call.name.as_str() {
        "create_session" => {
            let project = call.arguments.get("project")
                .and_then(|v| v.as_str())
                .ok_or_else(|| Error::invalid_params("missing project parameter"))?;
            session::create_session(mcp, project).await
        },
        "read_files" => handler_read::handle_read_files(mcp, &call).await,
        "list_directory" => handler_read::handle_list_directory(mcp, &call).await,
        "grep_file" => handler_grep::handle_grep_file(mcp, &call).await,
        "grep_codebase" => handler_grep::handle_grep_codebase(mcp, &call).await,
        "write_file_full" => handler_write::handle_write_file_full(mcp, &call).await,
        "write_file_small" => handler_write_small::handle_write_file_small(mcp, &call).await,
        "write_file_large" => handler_write::handle_write_file_large(mcp, &call).await,
        "run_command" => handler_command::handle_run_command(mcp, &call).await,
        "run_command_risky" => handler_command::handle_run_command_risky(mcp, &call).await,
        "run_test_command" => handler_command::handle_run_test_command(mcp, &call).await,
        _ => Ok(crate::error::format_error_response(format!("Unknown tool: {}", call.name)))
    }
}