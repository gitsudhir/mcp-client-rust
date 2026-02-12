use async_trait::async_trait;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::types::JsonRpcRequest;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Transport closed")]
    Closed,
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

pub type TransportResult<T> = Result<T, TransportError>;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: JsonRpcRequest) -> TransportResult<()>;
    async fn receive(&self) -> TransportResult<serde_json::Value>;
    async fn close(&self) -> TransportResult<()>;
}

pub struct StdioTransport {
    process: Arc<Mutex<Option<Child>>>,
    reader: Arc<Mutex<Box<dyn std::io::Read + Send>>>,
    writer: Arc<Mutex<Box<dyn std::io::Write + Send>>>,
}

impl StdioTransport {
    pub fn new(command: &str, args: &[&str]) -> TransportResult<Self> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdout = child.stdout.take().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdout")
        })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdin")
        })?;

        Ok(StdioTransport {
            process: Arc::new(Mutex::new(Some(child))),
            reader: Arc::new(Mutex::new(Box::new(stdout))),
            writer: Arc::new(Mutex::new(Box::new(stdin))),
        })
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send(&self, message: JsonRpcRequest) -> TransportResult<()> {
        let json = serde_json::to_string(&message)?;
        let mut writer = self.writer.lock().map_err(|_| {
            TransportError::ConnectionError("Failed to acquire writer lock".to_string())
        })?;
        writer.write_all(json.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    async fn receive(&self) -> TransportResult<serde_json::Value> {
        let mut reader = self.reader.lock().map_err(|_| {
            TransportError::ConnectionError("Failed to acquire reader lock".to_string())
        })?;
        let mut buf_reader = BufReader::new(&mut *reader);
        let mut line = String::new();
        
        if buf_reader.read_line(&mut line)? == 0 {
            return Err(TransportError::Closed);
        }

        Ok(serde_json::from_str(&line)?)
    }

    async fn close(&self) -> TransportResult<()> {
        if let Ok(mut process_guard) = self.process.lock() {
            if let Some(mut process) = process_guard.take() {
                process.kill()?;
            }
        }
        Ok(())
    }
}

pub struct HttpSSETransport {
    base_url: String,
    client: reqwest::Client,
    session_id: String,
}

impl HttpSSETransport {
    pub fn new(base_url: &str) -> Self {
        HttpSSETransport {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait]
impl Transport for HttpSSETransport {
    async fn send(&self, message: JsonRpcRequest) -> TransportResult<()> {
        let url = format!("{}/rpc", self.base_url);
        
        self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&message)
            .send()
            .await
            .map_err(|e| TransportError::ConnectionError(e.to_string()))?;

        Ok(())
    }

    async fn receive(&self) -> TransportResult<serde_json::Value> {
        let url = format!("{}/events/{}", self.base_url, self.session_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| TransportError::ConnectionError(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| TransportError::ConnectionError(e.to_string()))?;

        Ok(serde_json::from_str(&text)?)
    }

    async fn close(&self) -> TransportResult<()> {
        Ok(())
    }
}