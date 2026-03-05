use serde_json::Value;
use tracing::info;

use mcp_labs::application::tool_dispatch;
use mcp_labs::domain::ports::ProjectRepository;
use mcp_labs::infrastructure::process_runner::TokioProcessRunner;
use mcp_labs::infrastructure::stdio_transport::{read_line, write_response};
use mcp_labs::infrastructure::toml_store::EmbeddedTomlStore;
use mcp_labs::protocol::errors::McpError;
use mcp_labs::protocol::handlers::{handle_initialize, handle_ping, handle_tools_list};
use mcp_labs::protocol::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, ToolCallResult};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    info!("mcp-labs server starting");

    let store = EmbeddedTomlStore::new();
    let runner = TokioProcessRunner::new();

    while let Some(line) = read_line().await {
        if line.is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                let err = McpError::ParseError(e.to_string());
                let error_resp = JsonRpcError::new(Value::Null, err.code(), err.to_string());
                write_response(&error_resp).await;
                continue;
            }
        };

        let id = request.id.clone().unwrap_or(Value::Null);

        match request.method.as_str() {
            "initialize" => {
                write_response(&handle_initialize(id)).await;
            }
            "notifications/initialized" => {
                // Notification — no response
            }
            "ping" => {
                write_response(&handle_ping(id)).await;
            }
            "tools/list" => {
                write_response(&handle_tools_list(id)).await;
            }
            "tools/call" => {
                let params = request.params.unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let arguments = params
                    .get("arguments")
                    .cloned()
                    .unwrap_or(serde_json::json!({}));

                let registry = match store.load_registry().await {
                    Ok(r) => r,
                    Err(e) => {
                        let err = McpError::InternalError(e.to_string());
                        let error_resp = JsonRpcError::new(id, err.code(), err.to_string());
                        write_response(&error_resp).await;
                        continue;
                    }
                };

                match tool_dispatch::dispatch(tool_name, &arguments, &registry, &runner).await {
                    Ok(result) => {
                        let resp = JsonRpcResponse::new(id, serde_json::to_value(result).unwrap());
                        write_response(&resp).await;
                    }
                    Err(err) => {
                        let result = ToolCallResult::error(err.to_string());
                        let resp = JsonRpcResponse::new(id, serde_json::to_value(result).unwrap());
                        write_response(&resp).await;
                    }
                }
            }
            method => {
                let err = McpError::MethodNotFound(method.to_string());
                let error_resp = JsonRpcError::new(id, err.code(), err.to_string());
                write_response(&error_resp).await;
            }
        }
    }

    info!("mcp-labs server shutting down");
}
