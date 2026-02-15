use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::transport::Transport;
use mcp_client_rust::types::ClientInfo;
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MCP Client - Multi-Server Example");
    
    // Server 1: Main server
    let transport1 = Arc::new(StdioTransport::new(
        "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust",
        &[]
    )?) as Arc<dyn Transport>;
    
    let client_info1 = ClientInfo {
        name: "Server1Client".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client1 = MCPClient::new(transport1, client_info1);
    client1.initialize().await?;
    println!("✓ Connected to main server");

    // Test tools from server 1
    println!("\nServer 1 tools:");
    let tools1 = client1.list_tools().await?;
    for tool in tools1 {
        println!("- {} ({})", tool.name, tool.description.unwrap_or_default());
    }

    // Call a tool from server 1
    println!("\nCalling tool from server 1:");
    let result = client1.call_tool(
        "greet",
        json!({
            "name": "Multi-Server Test"
        })
    ).await?;
    
    match &result.content[0] {
        mcp_client_rust::types::ToolResultContent::Text { text } => {
            println!("Result: {}", text);
        }
        _ => println!("Received non-text result"),
    }

    client1.close().await?;
    println!("\n✓ Multi-server example completed successfully");
    Ok(())
}