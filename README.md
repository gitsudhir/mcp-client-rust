# MCP Client Rust - Model Context Protocol Client

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Status](https://img.shields.io/badge/Status-Active%20Development-green.svg)

A comprehensive Rust implementation of the **Model Context Protocol (MCP)** client. This project provides a robust, type-safe foundation for building AI applications that can interact with MCP-compatible servers.

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
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

### Core Features
- ✅ **Full MCP Protocol Support** - Complete implementation of JSON-RPC 2.0 based MCP
- ✅ **Type-Safe Client** - Leverages Rust's type system for compile-time safety
- ✅ **Async/Await** - Fully asynchronous using Tokio runtime
- ✅ **Multi-Server Support** - Manage multiple MCP server connections simultaneously
- ✅ **Tool Management** - Discover, validate, and execute server-exposed tools
- ✅ **Resource Access** - Read and manage resources from MCP servers
- ✅ **Prompt Support** - Utilize pre-defined LLM prompts from servers

### Security Features
- 🔒 **Input Validation** - Comprehensive input validation and sanitization
- 🔒 **Error Isolation** - Secure error handling preventing information leakage

### Advanced Features
- 📊 **Logging & Debugging** - Comprehensive structured logging system
- 🔄 **Reconnection Logic** - Automatic reconnection with exponential backoff
- 🛠️ **Tool Execution** - Execute MCP tools with validation and error handling
- 📱 **Streaming Support** - Handle streaming responses for long-running operations

## 📦 Prerequisites

### System Requirements
- **Rust**: 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Operating System**: macOS, Linux, or Windows

### Verify Installation

```bash
# Check Rust installation
rustc --version
cargo --version
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

### Step 3: Configure Environment

Create a `.env` file in the project root:

```env
# Logging
LOG_LEVEL=info
LOG_FILE=./mcp-client.log

# MCP Configuration
MCP_TIMEOUT_SECONDS=30
```

## 💡 Quick Start

### Basic Usage

```rust
use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::types::ClientInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize MCP client
    let transport = Box::new(StdioTransport::new("./mcp-server", &[])?);
    let client_info = ClientInfo {
        name: "MyClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;

    // List tools
    let tools = client.list_tools().await?;
    for tool in tools {
        println!("Tool: {} - {}", tool.name, tool.description.unwrap_or_default());
    }

    // Call a tool
    let result = client.call_tool(
        "greet",
        serde_json::json!({
            "name": "Alice"
        })
    ).await?;
    
    println!("Result: {:?}", result);

    client.close().await?;
    Ok(())
}
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
│   ├── tool_manager.rs        # Tool management and validation
│   ├── multi_server.rs        # Multi-server connection manager
│   ├── security.rs            # Security policies and validation
│   ├── logging.rs             # Logging utilities
│   ├── validation.rs          # Input validation
│   └── errors.rs              # Error types
├── examples/
│   ├── basic_example.rs
│   └── multi_server_example.rs
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
├── README.md
└── .env.example
```

## 📖 Usage Examples

### Example 1: Connect to Server and List Tools

```rust
use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::types::ClientInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create transport to connect to the server
    let transport = Box::new(StdioTransport::new(
        "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust",
        &[]
    )?);
    
    let client_info = ClientInfo {
        name: "TestClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;

    // List available tools
    let tools = client.list_tools().await?;
    println!("Available tools:");
    for tool in tools {
        println!("- {} ({})", tool.name, tool.description.unwrap_or_default());
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

    // Execute a greeting tool
    let result = client.call_tool(
        "greet",
        json!({
            "name": "Alice"
        })
    ).await?;

    match &result.content[0] {
        mcp_client_rust::types::ToolResultContent::Text { text } => {
            println!("Greeting result: {}", text);
        }
        _ => println!("Received non-text result"),
    }
    
    client.close().await?;
    Ok(())
}

async fn create_client() -> Result<MCPClient, Box<dyn std::error::Error>> {
    let transport = Box::new(StdioTransport::new(
        "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust",
        &[]
    )?);
    
    let client_info = mcp_client_rust::types::ClientInfo {
        name: "TestClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;
    Ok(client)
}
```

### Example 3: Read a Resource

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client().await?;

    // Read a resource
    let content = client.read_resource("config://app").await?;
    
    for item in content.contents {
        match item {
            mcp_client_rust::types::ContentItem::Text { text } => {
                println!("Content: {}", text);
            }
            mcp_client_rust::types::ContentItem::Blob { blob } => {
                println!("Binary data: {} bytes", blob.len());
            }
        }
    }

    client.close().await?;
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

## ⚙️ Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Logging Configuration
LOG_LEVEL=info
LOG_FILE=./mcp-client.log

# MCP Configuration
MCP_TIMEOUT_SECONDS=30
MCP_MAX_RETRIES=3
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

2. **Comprehensive Logging**
   ```rust
   let logger = mcp_client_rust::logging::McpLogger::new(mcp_client_rust::logging::LogLevel::Info)
       .with_file("./mcp-client.log".to_string());
   
   logger.info("Tool execution attempt");
   logger.error("Unauthorized access");
   ```

## 🐛 Troubleshooting

### Server Connection Issues

**Problem**: Cannot connect to MCP server

**Solutions**:
```rust
// Verify server path exists
let path = "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust";
if !std::path::Path::new(path).exists() {
    eprintln!("Server executable not found at: {}", path);
}

// Check server permissions
#[cfg(unix)]
std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
```

### Timeout Issues

**Problem**: "Request timeout" errors

**Solutions**:
```env
# Increase timeout
MCP_TIMEOUT_SECONDS=60
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
RUST_LOG=debug cargo run --example basic_example
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## 📚 Examples

All examples are located in the `examples/` directory:

1. **basic_example** - Simple tool execution
2. **multi_server_example** - Managing multiple MCP servers

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
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)

### Related Projects
- [MCP TypeScript SDK](https://github.com/modelcontextprotocol/typescript-sdk)
- [MCP Python SDK](https://github.com/modelcontextprotocol/python-sdk)
- [MCP Inspector](https://github.com/modelcontextprotocol/inspector)

## 📞 Support

For help and questions:
1. Check [Troubleshooting](#-troubleshooting) section
2. Search existing [GitHub Issues](https://github.com/yourusername/mcp-client-rust/issues)
3. Create a new issue with detailed information

## 🙏 Acknowledgments

- [Model Context Protocol](https://modelcontextprotocol.io/) team for the specification
- Rust community for amazing libraries and support

---

**Last Updated**: 2026-02-13  
**Version**: 0.1.0

Made with ❤️ in Rust