use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::transport::Transport;
use mcp_client_rust::types::ClientInfo;
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MCP Client connecting to server...");
    
    // Create transport to connect to the server
    let transport = Arc::new(StdioTransport::new(
        "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust",
        &[]
    )?) as Arc<dyn Transport>;
    
    let client_info = ClientInfo {
        name: "TestClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;
    println!("✓ Connected to server");

    // List available tools
    println!("\nAvailable tools:");
    let tools = client.list_tools().await?;
    for tool in tools {
        println!("- {} ({})", tool.name, tool.description.unwrap_or_default());
    }

    // Call greeting tool
    println!("\nCalling greeting tool:");
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

    // Calculate BMI
    println!("\nCalculating BMI:");
    let bmi_result = client.call_tool(
        "calculate-bmi",
        json!({
            "weightKg": 70.0,
            "heightM": 1.75
        })
    ).await?;
    
    match &bmi_result.content[0] {
        mcp_client_rust::types::ToolResultContent::Text { text } => {
            println!("BMI result: {}", text);
        }
        _ => println!("Received non-text result"),
    }

    // Fetch weather
    println!("\nFetching weather:");
    let weather_result = client.call_tool(
        "fetch-weather",
        json!({
            "city": "San Francisco"
        })
    ).await?;
    
    match &weather_result.content[0] {
        mcp_client_rust::types::ToolResultContent::Text { text } => {
            println!("Weather result: {}", text);
        }
        _ => println!("Received non-text result"),
    }

    // Read configuration resource
    println!("\nReading configuration resource:");
    let config_content = client.read_resource("config://app").await?;
    
    for item in config_content.contents {
        match item {
            mcp_client_rust::types::ContentItem::Text { text } => {
                println!("Config content: {}", text);
            }
            mcp_client_rust::types::ContentItem::Blob { blob } => {
                println!("Binary config data: {} bytes", blob.len());
            }
        }
    }

    // Get code review prompt
    println!("\nGetting code review prompt:");
    let prompt_result = client.get_prompt(
        "review-code",
        Some(std::collections::HashMap::from([
            ("code".to_string(), "fn main() { println!(\"Hello!\"); }".to_string()),
            ("focus".to_string(), "performance".to_string())
        ]))
    ).await?;
    
    for message in prompt_result.messages {
        println!("Role: {}", message.role);
        for content in message.content {
            match content {
                mcp_client_rust::types::MessageContent::Text { text } => {
                    println!("Prompt message: {}", text);
                }
                mcp_client_rust::types::MessageContent::Blob { blob } => {
                    println!("Binary prompt data: {} bytes", blob.len());
                }
            }
        }
    }


    client.close().await?;
    println!("\n✓ Client completed successfully");
    Ok(())
}