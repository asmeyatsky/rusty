use serde_json::Value;

use super::types::{
    InitializeResult, JsonRpcResponse, ServerCapabilities, ServerInfo, ToolsCapability,
    ToolsListResult,
};
use crate::tools;

pub fn handle_initialize(id: Value) -> JsonRpcResponse {
    let result = InitializeResult {
        protocol_version: "2025-11-25".to_string(),
        server_info: ServerInfo {
            name: "mcp-labs".to_string(),
            version: "0.1.0".to_string(),
        },
        capabilities: ServerCapabilities {
            tools: ToolsCapability {},
        },
    };
    JsonRpcResponse::new(id, serde_json::to_value(result).unwrap())
}

pub fn handle_ping(id: Value) -> JsonRpcResponse {
    JsonRpcResponse::new(id, serde_json::json!({}))
}

pub fn handle_tools_list(id: Value) -> JsonRpcResponse {
    let result = ToolsListResult {
        tools: tools::all_tool_descriptors(),
    };
    JsonRpcResponse::new(id, serde_json::to_value(result).unwrap())
}
