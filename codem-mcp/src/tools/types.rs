use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Value,
}