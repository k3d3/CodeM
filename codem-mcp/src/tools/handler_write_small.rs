use jsonrpc_stdio_server::jsonrpc_core::{Value, Result, Error};
use crate::{server::Mcp, tools::write};
use crate::tools::types::ToolCall;
use codem_core::types::{Change, LineRange};

pub async fn handle_write_file_small(mcp: &Mcp, call: &ToolCall) -> Result<Value> {
    let session_id = call.arguments.get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing session_id parameter"))?;

    let path = call.arguments.get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing path parameter"))?;

    let old_str = call.arguments.get("old_str")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing old_str in change"))?;
    
    let new_str = call.arguments.get("new_str")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::invalid_params("missing new_str in change"))?;

    let line_range = call.arguments.get("line_range").and_then(|v| {
        if v.is_null() {
            None
        } else {
            // Keep line numbers as 1-based since that's what codem-core expects
            let start = v.get("start")
                .and_then(|v| v.as_u64())
                .map(|n| n as usize);

            let end = v.get("end")
                .and_then(|v| v.as_u64())
                .map(|n| n as usize);

            Some(LineRange { start, end })
        }
    });

    let allow_multiple_matches = call.arguments.get("allow_multiple_matches").and_then(|v| v.as_bool()).unwrap_or(false);

    let change = Change {
        old_str: old_str.to_string(),
        new_str: new_str.to_string(),
        line_range,
        allow_multiple_matches,
    };


    let run_test = call.arguments.get("run_test")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    write::write_file_small(mcp, session_id, path, vec![change], run_test).await
}
