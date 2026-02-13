use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::types::ClientInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MCP Client - Basic Example");
    
    // Create transport to connect to the server
    let transport = Box::new(StdioTransport::new(
        "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust",
        &[]
    )?);
    
    let client_info = ClientInfo {
        name: "BasicClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;
    println!("✓ Connected to MCP server");

    // List available tools
    println!("\nAvailable tools:");
    let tools = client.list_tools().await?;
    for tool in tools {
        println!("- {} ({})", tool.name, tool.description.unwrap_or_default());
    }

    // List available resources
    println!("\nAvailable resources:");
    let (resources, _) = client.list_resources().await?;
    for resource in resources {
        println!("- {} ({})", resource.name, resource.uri);
    }

    // List available prompts
    println!("\nAvailable prompts:");
    let prompts = client.list_prompts().await?;
    for prompt in prompts {
        println!("- {} ({})", prompt.name, prompt.description.unwrap_or_default());
    }

    client.close().await?;
    println!("\n✓ Example completed successfully");
    Ok(())
}