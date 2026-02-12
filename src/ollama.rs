use serde::{Deserialize, Serialize};
use crate::errors::{McpError, McpResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: Option<bool>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub repeat_penalty: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub created_at: String,
    pub message: OllamaMessage,
    pub done: bool,
    #[serde(default)]
    pub total_duration: Option<u64>,
    #[serde(default)]
    pub load_duration: Option<u64>,
    #[serde(default)]
    pub prompt_eval_count: Option<i32>,
    #[serde(default)]
    pub prompt_eval_duration: Option<u64>,
    #[serde(default)]
    pub eval_count: Option<i32>,
    #[serde(default)]
    pub eval_duration: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    Text(String),
    ToolCall(ToolCall),
}

pub struct OllamaClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        OllamaClient {
            base_url: base_url.to_string(),
            model: model.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_message(
        &self,
        messages: Vec<OllamaMessage>,
        options: Option<OllamaOptions>,
    ) -> McpResult<OllamaResponse> {
        let opts = options.unwrap_or_default();

        let request = OllamaRequest {
            model: self.model.clone(),
            messages,
            stream: Some(false),
            temperature: opts.temperature,
            top_p: opts.top_p,
            top_k: opts.top_k,
            repeat_penalty: opts.repeat_penalty,
        };

        let response = self
            .client
            .post(&format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| McpError::LlmError(format!("Failed to send request: {}", e)))?;

        let result: OllamaResponse = response
            .json()
            .await
            .map_err(|e| McpError::LlmError(format!("Failed to parse response: {}", e)))?;

        Ok(result)
    }

    pub async fn list_models(&self) -> McpResult<Vec<String>> {
        let response = self
            .client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| McpError::LlmError(format!("Failed to list models: {}", e)))?;

        #[derive(Deserialize)]
        struct ModelsResponse {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let data: ModelsResponse = response
            .json()
            .await
            .map_err(|e| McpError::LlmError(format!("Failed to parse models: {}", e)))?;

        Ok(data.models.iter().map(|m| m.name.clone()).collect())
    }

    pub async fn health_check(&self) -> McpResult<bool> {
        match self
            .client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OllamaOptions {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub repeat_penalty: Option<f32>,
}

impl Default for OllamaOptions {
    fn default() -> Self {
        OllamaOptions {
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: Some(40),
            repeat_penalty: Some(1.1),
        }
    }
}