use serde_json::json;
use jsonrpc_stdio_server::jsonrpc_core::{Result, Value}; 
use std::path::PathBuf;
use crate::{server::Mcp, error::{error_response_with_content, get_error_content}};
use codem_core::types::{WriteResultDetails, Change};

pub async fn write_new_file(mcp: &Mcp, session_id: &str, path: &str, content: &str, run_test: bool) -> Result<Value> {
    match mcp.client.write_new_file(session_id, &PathBuf::from(path), content, run_test).await {
        Ok(result) => {
            let mut content = vec![json!({
                "type": "text",
                "text": format!(
                    "New file created successfully:\nPath: {}\nSize: {}\nLines: {}", 
                    path,
                    result.size,
                    result.line_count,
                )
            })];

            if let WriteResultDetails::WithTestOutput { output, .. } = &result.details {
                content.push(json!({
                    "type": "text",
                    "text": format!("\nTest output:\n{}\n", output)
                }));
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}

pub async fn write_file_full(mcp: &Mcp, session_id: &str, path: &str, content: &str, run_test: bool) -> Result<Value> {
    match mcp.client.write_file_full(session_id, &PathBuf::from(path), content, run_test).await {
        Ok(result) => {
            let mut content = vec![json!({
                "type": "text",
                "text": format!(
                    "File written successfully:\nPath: {}\nSize: {}\nLines: {}", 
                    path,
                    result.size,
                    result.line_count,
                )
            })];

            if let WriteResultDetails::WithTestOutput { output, .. } = &result.details {
                content.push(json!({
                    "type": "text",
                    "text": format!("\nTest output:\n{}\n", output)
                }));
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}

pub async fn write_file_small(mcp: &Mcp, session_id: &str, path: &str, changes: Vec<Change>, run_test: bool) -> Result<Value> {
    match mcp.client.write_file_partial(session_id, &PathBuf::from(path), changes, run_test).await {
        Ok(result) => {
            let mut content = vec![json!({
                "type": "text",
                "text": format!(
                    "File updated successfully:\nPath: {}\nSize: {}\nLines: {}\n",
                    path,
                    result.size,
                    result.line_count,
                )
            })];
            
            // Add change context information if available
            let write_details = match result.details {
                WriteResultDetails::None => None,
                WriteResultDetails::Partial(d) => Some(d),
                WriteResultDetails::WithTestOutput { output, details: d } => {
                    content.push(json!({
                        "type": "text",
                        "text": format!("\nTest output:\n{}\n", output)
                    }));
                    if let WriteResultDetails::Partial(d) = *d {
                        Some(d)
                    } else {
                        None
                    }
                },
                _ => None,
            };
            
            if let Some(partial_result) = write_details {
                for change_result in partial_result.change_results {
                    content.push(json!({
                        "type": "text",
                        "text": format!(
                            "\nChange at lines {}-{}:\n{}",
                            change_result.line_number_start,
                            change_result.line_number_end,
                            change_result.context
                        )
                    }));
                }
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}

pub async fn write_file_large(mcp: &Mcp, session_id: &str, path: &str, start_str: &str, end_str: &str, new_str: &str, run_test: bool) -> Result<Value> {
    match mcp.client.write_file_large(session_id, &PathBuf::from(path), start_str, end_str, new_str, run_test).await {
        Ok(result) => {
            let mut content = vec![json!({
                "type": "text",
                "text": format!(
                    "File section replaced successfully:\nPath: {}\nSize: {}\nLines: {}\n",
                    path,
                    result.size,
                    result.line_count,
                )
            })];
            
            // Add change context information if available
            let write_details = match result.details {
                WriteResultDetails::None => None,
                WriteResultDetails::PartialLarge(d) => Some(d),
                WriteResultDetails::WithTestOutput { output, details: d } => {
                    content.push(json!({
                        "type": "text",
                        "text": format!("\nTest output:\n{}\n", output)
                    }));
                    if let WriteResultDetails::PartialLarge(d) = *d {
                        Some(d)
                    } else {
                        None
                    }
                },
                _ => None,
            };
            
            if let Some(partial_result) = write_details {
                let context = &partial_result.context;
                content.push(json!({
                    "type": "text",
                    "text": format!(
                        "\nChange at lines {}-{}:\n--- Context before start ---\n{}\n--- Start content ---\n{}\n--- End content ---\n{}\n--- Context after end ---\n{}\n",
                        partial_result.line_number_start,
                        partial_result.line_number_end,
                        context.before_start.join("\n"),
                        context.start_content.join("\n"),
                        context.end_content.join("\n"),
                        context.after_end.join("\n")
                    )
                }));
            }

            Ok(json!({ "content": content }))
        },
        Err(e) => Ok(error_response_with_content(&e, get_error_content(&e)))
    }
}
