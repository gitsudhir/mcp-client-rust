use mcp_client_rust::multi_server::MultiServerManager;
use mcp_client_rust::logging::{LogLevel, McpLogger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = McpLogger::new(LogLevel::Info);
    logger.info("Multi-Server Manager Example");

    let _manager = MultiServerManager::new();
    
    println!("=== Multi-Server Manager ===");
    println!("This example demonstrates connecting to multiple MCP servers");
    println!("\nTo add servers:");
    println!("1. Create transport (stdio or HTTP SSE)");
    println!("2. Create ServerConfig with server details");
    println!("3. Call manager.add_server(transport, config)");
    println!("\nFeatures:");
    println!("- Connect to multiple MCP servers");
    println!("- Discover tools across all servers");
    println!("- Execute tools on specific servers");
    println!("- Manage server lifecycle");

    Ok(())
}