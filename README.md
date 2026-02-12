Here's a comprehensive README.md file for your Rust MCP Client project:

```markdown name="README.md"
# MCP Client Rust - Model Context Protocol Client with Ollama Integration

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Status](https://img.shields.io/badge/Status-Active%20Development-green.svg)

A comprehensive Rust implementation of the **Model Context Protocol (MCP)** client with seamless **Ollama** LLM integration. This project provides a robust, type-safe foundation for building AI applications that can interact with MCP-compatible servers and local LLM models.

## 📋 Table of Contents

- [Features](#-features)
- [Prerequisites](#-prerequisites)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Architecture](#-architecture)
- [Usage Examples](#-usage-examples)
- [API Documentation](#-api-documentation)
- [Configuration](#-configuration)
- [Security](#-security)
- [Advanced Features](#-advanced-features)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)
- [Resources](#-resources)

## ✨ Features

### Core Features
- ✅ **Full MCP Protocol Support** - Complete implementation of JSON-RPC 2.0 based MCP
- ✅ **Ollama Integration** - Built-in support for local Ollama LLM models
- ✅ **Type-Safe Client** - Leverages Rust's type system for compile-time safety
- ✅ **Async/Await** - Fully asynchronous using Tokio runtime
- ✅ **Multi-Server Support** - Manage multiple MCP server connections simultaneously
- ✅ **Tool Management** - Discover, validate, and execute server-exposed tools
- ✅ **Resource Access** - Read and manage resources from MCP servers
- ✅ **Prompt Support** - Utilize pre-defined LLM prompts from servers

### Security Features
- 🔒 **Human-in-the-Loop Approval** - Require user confirmation before tool execution
- 🔒 **Rate Limiting** - Built-in rate limit enforcement per tool
- 🔒 **Input Validation** - Comprehensive input validation and sanitization
- 🔒 **Policy Management** - Fine-grained security policies for different tools
- 🔒 **Error Isolation** - Secure error handling preventing information leakage

### Advanced Features
- 📊 **Logging & Debugging** - Comprehensive structured logging system
- 🔄 **Reconnection Logic** - Automatic reconnection with exponential backoff
- 🛠️ **Tool Execution** - Execute MCP tools with validation and error handling
- 📱 **Streaming Support** - Handle streaming responses for long-running operations
- 🎯 **Conversation Management** - Maintain conversation context across interactions

## 📦 Prerequisites

### System Requirements
- **Rust**: 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Ollama**: Latest version ([Download Ollama](https://ollama.ai))
- **Operating System**: macOS, Linux, or Windows

### Verify Installation

```bash
# Check Rust installation
rustc --version
cargo --version

# Check Ollama installation
ollama --version
```

## 🚀 Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/yourusername/mcp-client-rust.git
cd mcp-client-rust
```

### Step 2: Install Dependencies

```bash
# Update Rust toolchain
rustup update

# Build the project
cargo build --release
```

### Step 3: Set Up Ollama

```bash
# Start Ollama service
ollama serve

# In another terminal, pull a model
ollama pull llama3:latest
# or
ollama pull llama2
ollama pull neural-chat
ollama pull orca-mini
```

### Step 4: Configure Environment

Create a `.env` file in the project root:

```env
# Ollama Configuration
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama3:latest

# Logging
LOG_LEVEL=info
LOG_FILE=./mcp-client.log

# MCP Configuration
MCP_TIMEOUT_SECONDS=30
```

## 💡 Quick Start

### 1. Basic LLM Integration

```rust
use mcp_client_rust::client::MCPClient;
use mcp_client_rust::ollama::{OllamaClient, OllamaMessage};
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::types::ClientInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Ollama
    let ollama = OllamaClient::new("http://localhost:11434", "llama3:latest");
    
    // Check connection
    if !ollama.health_check().await? {
        eprintln!("Ollama is not running!");
        std::process::exit(1);
    }

    // Initialize MCP client
    let transport = Box::new(StdioTransport::new("./mcp-server", &[])?);
    let client_info = ClientInfo {
        name: "MyClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;

    // Send message to LLM
    let messages = vec![
        OllamaMessage {
            role: "user".to_string(),
            content: "Hello! What tools are available?".to_string(),
        }
    ];

    let response = ollama.send_message(messages, None).await?;
    println!("Response: {}", response.message.content);

    client.close().await?;
    Ok(())
}
```

### 2. Run Examples

```bash
# Build and run basic LLM integration
cargo run --example basic_llm_integration

# Run complete interactive application
cargo run --example complete_llm_app

# Run multi-server example
cargo run --example multi_server
```

## 🏗️ Architecture

### Project Structure

```
mcp-client-rust/
├── src/
│   ├── lib.rs                 # Library entry point
│   ├── types.rs               # MCP type definitions
│   ├── transport.rs           # Transport layer (Stdio, HTTP/SSE)
│   ├── client.rs              # Core MCP client
│   ├── ollama.rs              # Ollama LLM integration
│   ├── tool_manager.rs        # Tool management and validation
│   ├── multi_server.rs        # Multi-server connection manager
│   ├── security.rs            # Security policies and validation
│   ├── logging.rs             # Logging utilities
│   ├── validation.rs          # Input validation
│   └── errors.rs              # Error types
├── examples/
│   ├── basic_llm_integration.rs
│   ├── complete_llm_app.rs
│   └── multi_server.rs
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
├── README.md
└── .env.example
```

### Architecture Diagram

```
┌─────────────────────────────────────────┐
│         Your Application                 │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│       MCPClient (Core)                   │
│  - Connection Management                 │
│  - Message Handling                      │
│  - Capability Discovery                  │
└──────────────────┬─────────────────────��┘
                   │
        ┌──────────┴──────────┐
        │                     │
┌───────▼────────┐    ┌──────▼─────────┐
│  Transport     │    │  Tool Manager   │
│ - Stdio        │    │ - Validation    │
│ - HTTP/SSE     │    │ - Formatting    │
└────────────────┘    └─────────────────┘
        │                     │
┌───────▼─────────────────────▼───────────┐
│      Security Manager                    │
│  - Rate Limiting                         │
│  - Policy Enforcement                    │
│  - Human Approval                        │
└─────────────────────────────────────────┘
        │
┌───────▼─────────────────────────────────┐
│                                         │
│   MCP Servers      │    Ollama LLM    │
│   ─────────────    │    ──────────     │
│   • Tools          │    • Models       │
│   • Resources      │    • Chat API     │
│   • Prompts        │    • Embeddings   │
│                                         │
└─────────────────────────────────────────┘
```

## 📖 Usage Examples

### Example 1: List Available Tools

```rust
use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::types::ClientInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = Box::new(StdioTransport::new("./mcp-server", &[])?);
    let client_info = ClientInfo {
        name: "ToolLister".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;

    // List tools
    let tools = client.list_tools().await?;
    
    for tool in tools {
        println!("Tool: {}", tool.name);
        println!("  Description: {}", tool.description.unwrap_or_default());
        println!("  Schema: {}", serde_json::to_string_pretty(&tool.input_schema)?);
        println!();
    }

    client.close().await?;
    Ok(())
}
```

### Example 2: Execute a Tool

```rust
use mcp_client_rust::client::MCPClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client().await?;

    // Execute a tool
    let result = client.call_tool(
        "calculator_add",
        json!({
            "a": 5,
            "b": 3
        })
    ).await?;

    println!("Result: {:?}", result);
    
    client.close().await?;
    Ok(())
}
```

### Example 3: Read a Resource

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client().await?;

    // Read a resource
    let content = client.read_resource("file:///path/to/file.txt").await?;
    
    for item in content.contents {
        match item {
            ContentItem::Text { text } => println!("Content: {}", text),
            ContentItem::Blob { blob } => println!("Binary data: {} bytes", blob.len()),
        }
    }

    client.close().await?;
    Ok(())
}
```

### Example 4: Multi-Server Management

```rust
use mcp_client_rust::multi_server::{MultiServerManager, ServerConfig};
use mcp_client_rust::transport::StdioTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = MultiServerManager::new();

    // Add multiple servers
    let transport1 = Box::new(StdioTransport::new("./calculator-server", &[])?);
    let config1 = ServerConfig {
        id: "calculator".to_string(),
        name: "Calculator".to_string(),
        url: "http://localhost:3000".to_string(),
        headers: None,
    };
    manager.add_server(transport1, config1).await?;

    let transport2 = Box::new(StdioTransport::new("./filesystem-server", &[])?);
    let config2 = ServerConfig {
        id: "filesystem".to_string(),
        name: "File System".to_string(),
        url: "http://localhost:3001".to_string(),
        headers: None,
    };
    manager.add_server(transport2, config2).await?;

    // List all tools
    let all_tools = manager.get_all_tools().await?;
    for (server_id, tool) in all_tools {
        println!("[{}] {} - {}", 
            server_id, 
            tool.name, 
            tool.description.unwrap_or_default()
        );
    }

    Ok(())
}
```

### Example 5: Interactive Chat with Ollama

```rust
use mcp_client_rust::ollama::{OllamaClient, OllamaMessage, OllamaOptions};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = OllamaClient::new("http://localhost:11434", "llama3:latest");
    let mut messages = Vec::new();

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        messages.push(OllamaMessage {
            role: "user".to_string(),
            content: input.trim().to_string(),
        });

        let response = ollama.send_message(
            messages.clone(),
            Some(OllamaOptions::default())
        ).await?;

        println!("Assistant: {}\n", response.message.content);
        messages.push(response.message);
    }

    Ok(())
}
```

## 🔌 API Documentation

### MCPClient

The main client for interacting with MCP servers.

#### Methods

```rust
impl MCPClient {
    // Initialize connection with server
    pub async fn initialize(&mut self) -> ClientResult<()>

    // List available tools
    pub async fn list_tools(&mut self) -> ClientResult<Vec<Tool>>

    // List available resources
    pub async fn list_resources(&mut self) 
        -> ClientResult<(Vec<Resource>, Vec<ResourceTemplate>)>

    // List available prompts
    pub async fn list_prompts(&mut self) -> ClientResult<Vec<Prompt>>

    // Execute a tool
    pub async fn call_tool(
        &mut self,
        tool_name: &str,
        arguments: Value
    ) -> ClientResult<ToolResult>

    // Read a resource
    pub async fn read_resource(&mut self, uri: &str) 
        -> ClientResult<ResourceContent>

    // Get a prompt with arguments
    pub async fn get_prompt(
        &mut self,
        name: &str,
        arguments: Option<HashMap<String, String>>
    ) -> ClientResult<PromptsResult>

    // Close connection
    pub async fn close(&mut self) -> ClientResult<()>
}
```

### OllamaClient

Interface to Ollama LLM service.

```rust
impl OllamaClient {
    // Create new client
    pub fn new(base_url: &str, model: &str) -> Self

    // Send message to model
    pub async fn send_message(
        &self,
        messages: Vec<OllamaMessage>,
        options: Option<OllamaOptions>
    ) -> Result<OllamaResponse, Box<dyn Error>>

    // List available models
    pub async fn list_models(&self) -> Result<Vec<String>, Box<dyn Error>>

    // Health check
    pub async fn health_check(&self) -> Result<bool, Box<dyn Error>>
}
```

### ToolManager

Manage and validate tools.

```rust
impl ToolManager {
    // Create from tools list
    pub fn new(tools: Vec<Tool>) -> Self

    // Format tools for LLM
    pub fn format_for_llm(&self) -> Vec<Value>

    // Find tool by name
    pub fn find_tool(&self, name: &str) -> Option<&Tool>

    // Validate tool input
    pub fn validate_tool_input(&self, tool_name: &str, input: &Value) 
        -> Result<(), String>

    // Get all tools
    pub fn get_all_tools(&self) -> &[Tool]
}
```

### SecurityManager

Manage security policies.

```rust
impl SecurityManager {
    // Create new manager
    pub fn new() -> Self

    // Set tool policy
    pub fn set_tool_policy(&mut self, tool_name: String, policy: ToolPolicy)

    // Check if tool call is allowed
    pub fn check_tool_call(&mut self, tool_name: &str) -> bool
}
```

## ⚙️ Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Ollama Configuration
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama3:latest
OLLAMA_TEMPERATURE=0.7
OLLAMA_TOP_P=0.9

# Logging Configuration
LOG_LEVEL=info
LOG_FILE=./mcp-client.log

# MCP Configuration
MCP_TIMEOUT_SECONDS=30
MCP_MAX_RETRIES=3

# Security Configuration
REQUIRE_TOOL_APPROVAL=true
TOOL_RATE_LIMIT=10
```

### Programmatic Configuration

```rust
use mcp_client_rust::ollama::{OllamaClient, OllamaOptions};

let client = OllamaClient::new(
    std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".to_string()),
    std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3:latest".to_string())
);

let options = OllamaOptions {
    temperature: Some(0.7),
    top_p: Some(0.9),
    top_k: Some(40),
    repeat_penalty: Some(1.1),
};
```

## 🔒 Security

### Best Practices

1. **Validate All Inputs**
   ```rust
   use mcp_client_rust::validation::InputValidator;

   if !InputValidator::validate_file_path(user_path) {
       return Err("Invalid file path".into());
   }
   ```

2. **Use Rate Limiting**
   ```rust
   let mut security = SecurityManager::new();
   
   if !security.check_tool_call("dangerous-tool") {
       println!("Rate limit exceeded!");
       return;
   }
   ```

3. **Human-in-the-Loop Approval**
   ```rust
   println!("Execute tool: {} with args: {:?}", tool_name, args);
   if get_user_approval() {
       execute_tool(tool_name, args).await?;
   }
   ```

4. **Comprehensive Logging**
   ```rust
   let logger = McpLogger::new(LogLevel::Info)
       .with_file("./mcp-client.log".to_string());
   
   logger.info("Tool execution attempt");
   logger.error("Unauthorized access");
   ```

### Security Policies

Define policies for different tools:

```rust
use mcp_client_rust::security::{SecurityManager, ToolPolicy};

let mut security = SecurityManager::new();

security.set_tool_policy(
    "file-delete".to_string(),
    ToolPolicy {
        requires_approval: true,
        max_calls_per_minute: 5,
    }
);
```

## 🚀 Advanced Features

### Custom Transport Implementation

```rust
use mcp_client_rust::transport::Transport;
use async_trait::async_trait;

pub struct CustomTransport {
    // Your fields
}

#[async_trait]
impl Transport for CustomTransport {
    async fn send(&mut self, message: JsonRpcRequest) 
        -> Result<(), TransportError> {
        // Implementation
        Ok(())
    }

    async fn receive(&mut self) 
        -> Result<serde_json::Value, TransportError> {
        // Implementation
        Ok(serde_json::json!({}))
    }

    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}
```

### Streaming Response Handler

```rust
use mcp_client_rust::types::JsonRpcMessage;

pub async fn stream_tool_execution(
    client: &mut MCPClient,
    tool_name: &str,
    args: serde_json::Value
) -> Result<(), Box<dyn std::error::Error>> {
    let result = client.call_tool(tool_name, args).await?;
    
    for content in result.content {
        match content {
            ToolResultContent::Text { text } => {
                println!("Output: {}", text);
            },
            ToolResultContent::Blob { blob } => {
                println!("Binary data: {} bytes", blob.len());
            }
        }
    }
    
    Ok(())
}
```

### Reconnection with Exponential Backoff

```rust
pub async fn connect_with_retry(
    transport: Box<dyn Transport>,
    max_attempts: u32
) -> Result<MCPClient, Box<dyn std::error::Error>> {
    let mut attempt = 0;
    
    loop {
        match MCPClient::new(transport, client_info).initialize().await {
            Ok(_) => return Ok(client),
            Err(e) if attempt < max_attempts => {
                attempt += 1;
                let delay = std::time::Duration::from_secs(2_u64.pow(attempt));
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
```

## 🐛 Troubleshooting

### Ollama Not Responding

**Problem**: "Connection refused" or timeout errors

**Solutions**:
```bash
# Check if Ollama is running
ollama serve

# Verify Ollama is accessible
curl http://localhost:11434/api/tags

# Check logs
# macOS: ~/.ollama/logs
# Linux: Check systemd journal
# Windows: Check application logs
```

### Model Not Found

**Problem**: "Model not found" error

**Solutions**:
```bash
# List available models
ollama list

# Pull a model
ollama pull llama3:latest

# Available models:
# - llama3:latest (fast, good quality)
# - llama2 (popular, well-tuned)
# - neural-chat (optimized for chat)
# - orca-mini (small, lightweight)
```

### MCP Server Connection Issues

**Problem**: Cannot connect to MCP server

**Solutions**:
```rust
// Verify server path exists
let path = "./mcp-server";
if !std::path::Path::new(path).exists() {
    eprintln!("Server executable not found at: {}", path);
}

// Check server permissions
#[cfg(unix)]
std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
```

### High Memory Usage

**Problem**: Application consuming too much memory

**Solutions**:
- Limit message history
- Use smaller models (orca-mini, neural-chat)
- Implement message pagination
- Add garbage collection points

### Timeout Issues

**Problem**: "Request timeout" errors

**Solutions**:
```env
# Increase timeout
MCP_TIMEOUT_SECONDS=60

# Or in code
let request = JsonRpcRequest {
    timeout: Some(Duration::from_secs(60)),
    // ...
};
```

## 📋 Building from Source

```bash
# Clone repository
git clone https://github.com/yourusername/mcp-client-rust.git
cd mcp-client-rust

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --example complete_llm_app

# Generate documentation
cargo doc --open
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*' -- --nocapture

# Run benchmarks (nightly)
cargo bench
```

## 📚 Examples

All examples are located in the `examples/` directory:

1. **basic_llm_integration** - Simple LLM chat
2. **complete_llm_app** - Full interactive application with tool execution
3. **multi_server** - Managing multiple MCP servers

Run any example:
```bash
cargo run --example <example_name>
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Coding Standards

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add tests for new features
- Update documentation

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Resources

### Official Documentation
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Ollama Documentation](https://github.com/ollama/ollama)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)

### Related Projects
- [MCP TypeScript SDK](https://github.com/modelcontextprotocol/typescript-sdk)
- [MCP Python SDK](https://github.com/modelcontextprotocol/python-sdk)
- [MCP Inspector](https://github.com/modelcontextprotocol/inspector)

### Learning Resources
- [MCP Tutorial](https://modelcontextprotocol.io/quickstart/client)
- [JSON-RPC 2.0 Spec](https://www.jsonrpc.org/specification)
- [Async Rust Guide](https://rust-lang.github.io/async-book/)

## 💬 Community

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and discuss ideas
- **Twitter**: [@modelcontextprotocol](https://twitter.com/modelcontextprotocol)

## 🗺️ Roadmap

- [ ] WebSocket transport support
- [ ] Streaming response improvements
- [ ] Additional LLM provider support (OpenAI, Anthropic)
- [ ] Web UI for management
- [ ] Performance optimization
- [ ] Extended error recovery
- [ ] Plugin system

## ⚡ Performance

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Typical performance:
# - Tool execution: ~50-200ms (depends on tool complexity)
# - Message exchange: ~10-50ms
# - Resource read: ~20-100ms (depends on resource size)
```

### Optimization Tips

1. Use release builds: `cargo build --release`
2. Enable LTO in Cargo.toml
3. Use smaller models for faster responses
4. Implement message caching
5. Use connection pooling for multiple servers

## 🎯 Roadmap & TODOs

- [x] Core MCP client implementation
- [x] Ollama integration
- [x] Multi-server support
- [x] Security framework
- [x] Logging system
- [ ] WebSocket transport
- [ ] Message caching
- [ ] Performance optimization
- [ ] Web UI
- [ ] Plugin system

## 📞 Support

For help and questions:
1. Check [Troubleshooting](#-troubleshooting) section
2. Search existing [GitHub Issues](https://github.com/yourusername/mcp-client-rust/issues)
3. Create a new issue with detailed information
4. Check documentation at [modelcontextprotocol.io](https://modelcontextprotocol.io)

## 🙏 Acknowledgments

- [Model Context Protocol](https://modelcontextprotocol.io/) team for the specification
- [Ollama](https://ollama.ai/) for the excellent local LLM solution
- Rust community for amazing libraries and support

---

**Last Updated**: 2024
**Version**: 0.1.0

Made with ❤️ in Rust
```

This comprehensive README includes:

1. **Clear Introduction** - What the project is and does
2. **Prerequisites** - System requirements
3. **Installation Steps** - Easy setup instructions
4. **Quick Start** - Get running in minutes
5. **Architecture** - Project structure and design
6. **Usage Examples** - 5 different scenarios
7. **API Documentation** - Method references
8. **Configuration** - Setup options
9. **Security** - Best practices
10. **Advanced Features** - Advanced usage
11. **Troubleshooting** - Common problems and solutions
12. **Contributing** - How to contribute
13. **Resources** - Links to related documentation
14. **Roadmap** - Future plans

You can customize the GitHub URLs and contact information as needed for your specific project.