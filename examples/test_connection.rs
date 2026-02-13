use mcp_client_rust::{
    client::MCPClient,
    transport::StdioTransport,
    types::{ClientInfo, Tool},
};
use std::sync::Arc;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing MCP Client-Server Connection...");
    
    // Create client info
    let client_info = ClientInfo {
        name: "TestClient".to_string(),
        version: "1.0.0".to_string(),
    };
    
    // Create stdio transport pointing to the server
    let server_path = "../mcp-server-rust/target/release/mcp-server-rust";
    
    println!("Connecting to server at: {}", server_path);
    
    let transport = match StdioTransport::new(server_path, &[]) {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Failed to create transport: {}", e);
            return Err(e.into());
        }
    };
    
    // Create client
    let mut client = MCPClient::new(transport, client_info);
    
    // Initialize connection with timeout
    println!("Initializing connection...");
    match timeout(Duration::from_secs(10), client.initialize()).await {
        Ok(Ok(())) => println!("✓ Connection initialized successfully"),
        Ok(Err(e)) => {
            eprintln!("✗ Failed to initialize: {}", e);
            return Err(e.into());
        }
        Err(_) => {
            eprintln!("✗ Connection timeout");
            return Err("Connection timeout".into());
        }
    }
    
    // List available tools
    println!("Listing available tools...");
    match timeout(Duration::from_secs(5), client.list_tools()).await {
        Ok(Ok(tools)) => {
            println!("✓ Found {} tools:", tools.len());
            for tool in tools {
                println!("  - {}: {}", tool.name, tool.description.unwrap_or_default());
            }
        }
        Ok(Err(e)) => {
            eprintln!("✗ Failed to list tools: {}", e);
            return Err(e.into());
        }
        Err(_) => {
            eprintln!("✗ Tools listing timeout");
            return Err("Tools listing timeout".into());
        }
    }
    
    // Test calling a tool
    println!("Testing tool call (greet)...");
    let args = serde_json::json!({"name": "Test User"});
    
    match timeout(Duration::from_secs(5), client.call_tool("greet", args)).await {
        Ok(Ok(result)) => {
            println!("✓ Tool call successful:");
            for content in result.content {
                match content {
                    mcp_client_rust::types::ToolResultContent::Text { text } => {
                        println!("  Result: {}", text);
                    }
                    mcp_client_rust::types::ToolResultContent::Blob { blob } => {
                        println!("  Binary result: {} bytes", blob.len());
                    }
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("✗ Tool call failed: {}", e);
            return Err(e.into());
        }
        Err(_) => {
            eprintln!("✗ Tool call timeout");
            return Err("Tool call timeout".into());
        }
    }
    
    // Close connection
    println!("Closing connection...");
    client.close().await?;
    println!("✓ Connection closed successfully");
    
    println!("\n🎉 All tests passed! Client successfully connected to server.");
    Ok(())
}