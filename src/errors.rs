use thiserror::Error;
use crate::client::ClientError;
use crate::transport::TransportError;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Transport error: {0}")]
    Transport(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Tool execution error: {0}")]
    ToolExecution(String),
    #[error("Security violation: {0}")]
    SecurityViolation(String),
    #[error("LLM error: {0}")]
    LlmError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Client error: {0}")]
    ClientError(#[from] ClientError),
}

pub type McpResult<T> = Result<T, McpError>;

// IMPORTANT: Add this conversion implementation
impl From<TransportError> for McpError {
    fn from(err: TransportError) -> Self {
        McpError::Transport(format!("{:?}", err))
    }
}
