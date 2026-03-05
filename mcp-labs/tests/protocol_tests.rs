use serde_json::{json, Value};

use mcp_labs::protocol::errors::McpError;
use mcp_labs::protocol::handlers::{handle_initialize, handle_ping, handle_tools_list};
use mcp_labs::protocol::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

fn parse_request(s: &str) -> JsonRpcRequest {
    serde_json::from_str(s).unwrap()
}

fn response_to_value(resp: &JsonRpcResponse) -> Value {
    serde_json::to_value(resp).unwrap()
}

fn error_to_value(resp: &JsonRpcError) -> Value {
    serde_json::to_value(resp).unwrap()
}

// ── Initialize ──

#[test]
fn initialize_returns_protocol_version() {
    let resp = handle_initialize(json!(1));
    let val = response_to_value(&resp);
    assert_eq!(val["result"]["protocolVersion"], "2025-11-25");
}

#[test]
fn initialize_returns_server_info() {
    let resp = handle_initialize(json!(1));
    let val = response_to_value(&resp);
    assert_eq!(val["result"]["serverInfo"]["name"], "mcp-labs");
    assert_eq!(val["result"]["serverInfo"]["version"], "0.1.0");
}

#[test]
fn initialize_returns_capabilities() {
    let resp = handle_initialize(json!(1));
    let val = response_to_value(&resp);
    assert!(val["result"]["capabilities"]["tools"].is_object());
}

#[test]
fn initialize_preserves_id() {
    let resp = handle_initialize(json!(42));
    let val = response_to_value(&resp);
    assert_eq!(val["id"], 42);
}

// ── Ping ──

#[test]
fn ping_returns_empty_object() {
    let resp = handle_ping(json!(99));
    let val = response_to_value(&resp);
    assert_eq!(val["result"], json!({}));
    assert_eq!(val["id"], 99);
}

// ── Tools List ──

#[test]
fn tools_list_returns_four_tools() {
    let resp = handle_tools_list(json!(2));
    let val = response_to_value(&resp);
    let tools = val["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 4);
}

#[test]
fn tools_list_contains_expected_names() {
    let resp = handle_tools_list(json!(2));
    let val = response_to_value(&resp);
    let tools = val["result"]["tools"].as_array().unwrap();
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"list_projects"));
    assert!(names.contains(&"get_project_status"));
    assert!(names.contains(&"search_projects"));
    assert!(names.contains(&"stack_analysis"));
}

#[test]
fn tools_have_input_schemas() {
    let resp = handle_tools_list(json!(2));
    let val = response_to_value(&resp);
    let tools = val["result"]["tools"].as_array().unwrap();
    for tool in tools {
        assert!(
            tool["inputSchema"].is_object(),
            "Tool {} missing inputSchema",
            tool["name"]
        );
    }
}

// ── JSON-RPC format ──

#[test]
fn response_has_jsonrpc_field() {
    let resp = handle_ping(json!(1));
    let val = response_to_value(&resp);
    assert_eq!(val["jsonrpc"], "2.0");
}

#[test]
fn error_response_format() {
    let err = McpError::MethodNotFound("foo".to_string());
    let resp = JsonRpcError::new(json!(5), err.code(), err.to_string());
    let val = error_to_value(&resp);
    assert_eq!(val["jsonrpc"], "2.0");
    assert_eq!(val["id"], 5);
    assert_eq!(val["error"]["code"], -32601);
    assert!(val["error"]["message"].as_str().unwrap().contains("foo"));
}

// ── Request parsing ──

#[test]
fn parse_valid_request() {
    let req = parse_request(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#);
    assert_eq!(req.method, "initialize");
    assert_eq!(req.id, Some(json!(1)));
}

#[test]
fn parse_request_without_id() {
    let req = parse_request(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    assert_eq!(req.method, "notifications/initialized");
    assert!(req.id.is_none());
}

#[test]
fn parse_request_with_params() {
    let req = parse_request(
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list_projects","arguments":{}}}"#,
    );
    assert_eq!(req.method, "tools/call");
    let params = req.params.unwrap();
    assert_eq!(params["name"], "list_projects");
}

#[test]
fn malformed_json_fails_parse() {
    let result: Result<JsonRpcRequest, _> = serde_json::from_str("not json");
    assert!(result.is_err());
}

// ── Error codes ──

#[test]
fn error_codes_are_correct() {
    assert_eq!(McpError::ParseError("".into()).code(), -32700);
    assert_eq!(McpError::InvalidRequest("".into()).code(), -32600);
    assert_eq!(McpError::MethodNotFound("".into()).code(), -32601);
    assert_eq!(McpError::InvalidParams("".into()).code(), -32602);
    assert_eq!(McpError::InternalError("".into()).code(), -32603);
    assert_eq!(McpError::ToolError("".into()).code(), -32000);
}
