# MCP Client-Server Integration Guide

This guide explains how to use the Rust MCP client to connect to the Rust MCP server.

## Prerequisites

1. **Rust** (1.70 or later) - [Install Rust](https://www.rust-lang.org/tools/install)
2. Both the client and server repositories should be cloned and built

## Project Structure

```
mcp-server-rust/          # Server repository
├── src/
│   ├── main.rs          # Server entry point
│   └── lib.rs           # Server library
└── target/release/mcp-server-rust  # Built server executable

mcp-client-rust/          # Client repository
├── src/
│   ├── main.rs          # Client entry point
│   ├── lib.rs           # Client library
│   └── examples/        # Example applications
└── target/release/      # Built client executables
```

## Building the Projects

### Build the Server

```bash
cd /Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust
cargo build --release
```

### Build the Client

```bash
cd /Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-client-rust
cargo build --release
```

## Running the Integration

### 1. Test Connection

Run the connection test to verify everything works:

```bash
cd /Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-client-rust
cargo run --example test_connection --release
```

This will:
- Connect to the server
- Initialize the connection
- List available tools
- Test calling a tool
- Close the connection

### 2. Basic Example

Run the basic example for simple usage:

```bash
cd /Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-client-rust
cargo run --example basic_example --release
```

### 3. Multi-Server Example

Run the multi-server example to see how to manage multiple server connections:

```bash
cd /Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-client-rust
cargo run --example multi_server_example --release
```

## Available Tools

The server provides the following tools:

1. **greet** - Greets a user with a personalized message
   - Parameters: `name` (string)
   - Example: `{"name": "Alice"}`

2. **calculate-bmi** - Calculates Body Mass Index
   - Parameters: `weightKg` (number), `heightM` (number)
   - Example: `{"weightKg": 70, "heightM": 1.75}`

3. **fetch-weather** - Fetches weather information (simulated)
   - Parameters: `city` (string)
   - Example: `{"city": "San Francisco"}`

## Available Resources

1. **config://app** - Application configuration
   - Returns JSON configuration data

## Available Prompts

1. **review-code** - Code review prompt generator
   - Parameters: `code` (required), `focus` (optional)
   - Example: `{"code": "fn main() { println!(\"Hello\"); }", "focus": "performance"}`

## Using the Client Library

### Basic Usage

```rust
use mcp_client_rust::{
    client::MCPClient,
    transport::StdioTransport,
    types::ClientInfo,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client info
    let client_info = ClientInfo {
        name: "MyClient".to_string(),
        version: "1.0.0".to_string(),
    };
    
    // Create transport (connects to server)
    let transport = Arc::new(StdioTransport::new(
        "../mcp-server-rust/target/release/mcp-server-rust",
        &[]
    )?);
    
    // Create client
    let mut client = MCPClient::new(transport, client_info);
    
    // Initialize connection
    client.initialize().await?;
    
    // List tools
    let tools = client.list_tools().await?;
    println!("Available tools: {:?}", tools);
    
    // Call a tool
    let result = client.call_tool("greet", serde_json::json!({"name": "World"})).await?;
    println!("Result: {:?}", result);
    
    // Close connection
    client.close().await?;
    
    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **"No such file or directory" error**
   - Make sure both projects are built with `cargo build --release`
   - Verify the server executable path is correct

2. **Connection timeout**
   - Check if the server is running properly
   - Ensure the server path in the client code is correct

3. **Tool not found errors**
   - Verify the tool name matches exactly
   - Check that the server supports the requested tool

### Debugging

Enable debug logging:

```bash
RUST_LOG=debug cargo run --example test_connection --release
```

## Next Steps

1. Explore the examples in the `examples/` directory
2. Review the API documentation in the source code
3. Create your own custom tools and integrate them
4. Extend the client with additional features as needed

## API Reference

For detailed API documentation, refer to the source code comments and the `src/` directory structure.