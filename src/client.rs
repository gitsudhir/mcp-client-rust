use crate::transport::Transport;
use crate::types::*;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Transport error: {0}")]
    TransportError(#[from] crate::transport::TransportError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct MCPClient {
    transport: Arc<dyn Transport>,
    client_info: ClientInfo,
    server_info: Option<ServerInfo>,
    request_id: Arc<AtomicU64>,
}

impl MCPClient {
    pub fn new(
        transport: Arc<dyn Transport>,
        client_info: ClientInfo,
    ) -> Self {
        MCPClient {
            transport,
            client_info,
            server_info: None,
            request_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub async fn initialize(&mut self) -> ClientResult<()> {
        let params = json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": {},
                "prompts": {}
            },
            "clientInfo": {
                "name": self.client_info.name,
                "version": self.client_info.version
            }
        });

        let response = self.send_request("initialize", Some(params)).await?;

        if let Some(server_info) = response.get("serverInfo") {
            self.server_info = serde_json::from_value(server_info.clone()).ok();
        }

        Ok(())
    }

    pub async fn list_tools(&mut self) -> ClientResult<Vec<Tool>> {
        let response = self.send_request("tools/list", None).await?;
        let tools: Vec<Tool> = serde_json::from_value(response.get("tools").cloned().unwrap_or(Value::Array(vec![])))?;
        Ok(tools)
    }

    pub async fn list_resources(&mut self) -> ClientResult<(Vec<Resource>, Vec<ResourceTemplate>)> {
        let response = self.send_request("resources/list", None).await?;
        let resources: Vec<Resource> = serde_json::from_value(response.get("resources").cloned().unwrap_or(Value::Array(vec![])))?;
        let templates: Vec<ResourceTemplate> = serde_json::from_value(response.get("resourceTemplates").cloned().unwrap_or(Value::Array(vec![])))?;
        Ok((resources, templates))
    }

    pub async fn list_prompts(&mut self) -> ClientResult<Vec<Prompt>> {
        let response = self.send_request("prompts/list", None).await?;
        let prompts: Vec<Prompt> = serde_json::from_value(response.get("prompts").cloned().unwrap_or(Value::Array(vec![])))?;
        Ok(prompts)
    }

    pub async fn call_tool(&mut self, tool_name: &str, arguments: Value) -> ClientResult<ToolResult> {
        let params = json!({
            "name": tool_name,
            "arguments": arguments
        });

        let response = self.send_request("tools/call", Some(params)).await?;
        let result: ToolResult = serde_json::from_value(response)?;
        Ok(result)
    }

    pub async fn read_resource(&mut self, uri: &str) -> ClientResult<ResourceContent> {
        let params = json!({
            "uri": uri
        });

        let response = self.send_request("resources/read", Some(params)).await?;
        let content: ResourceContent = serde_json::from_value(response)?;
        Ok(content)
    }

    pub async fn get_prompt(&mut self, name: &str, arguments: Option<std::collections::HashMap<String, String>>) -> ClientResult<PromptsResult> {
        let params = json!({
            "name": name,
            "arguments": arguments.unwrap_or_default()
        });

        let response = self.send_request("prompts/get", Some(params)).await?;
        let result: PromptsResult = serde_json::from_value(response)?;
        Ok(result)
    }

    async fn send_request(&mut self, method: &str, params: Option<Value>) -> ClientResult<Value> {
        let request_id = self.request_id.fetch_add(1, Ordering::SeqCst);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(request_id),
            method: method.to_string(),
            params,
        };

        // FIXED: Now uses the From<TransportError> implementation
        self.transport.send(request).await?;
        let response = self.transport.receive().await?;
        
        if let Some(result) = response.get("result") {
            Ok(result.clone())
        } else if let Some(error) = response.get("error") {
            Err(ClientError::RequestFailed(
                error.get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string()
            ))
        } else {
            Err(ClientError::RequestFailed("No result or error in response".to_string()))
        }
    }

    pub async fn close(&mut self) -> ClientResult<()> {
        // FIXED: Now uses the From<TransportError> implementation
        self.transport.close().await?;
        Ok(())
    }
}