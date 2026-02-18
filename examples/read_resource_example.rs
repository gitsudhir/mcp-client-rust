use mcp_client_rust::client::MCPClient;
use mcp_client_rust::transport::StdioTransport;
use mcp_client_rust::transport::Transport;
use mcp_client_rust::types::ClientInfo;
use mcp_client_rust::types::ContentItem;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MCP Client - Read Resource Example");

    // --- MCP Client Setup ---
    let transport = Arc::new(StdioTransport::new(
        "/Users/sudhirkumar/Desktop/sudhir/gitsudhir/mcp-server-rust/target/release/mcp-server-rust",
        &[],
    )?) as Arc<dyn Transport>;

    let client_info = ClientInfo {
        name: "ReadResourceClient".to_string(),
        version: "1.0.0".to_string(),
    };

    let mut client = MCPClient::new(transport, client_info);
    client.initialize().await?;
    println!("✓ Connected to MCP server");

    // --- Read File Resource ---
    // 1. Get the absolute path to the file.
    let file_uri = "file:///data/example_file.txt".to_string();

    println!("\nAttempting to read resource from: {}", file_uri);

    // 3. Call the read_resource method.
    let resource_content = client.read_resource(&file_uri).await?;

    // 4. Print
    // The content is available in the `content` field of the returned struct.
    if !resource_content.contents.is_empty() {
        println!("\n✓ File content retrieved successfully:\n---");
        for item in resource_content.contents {
            match item {
                ContentItem::Text { text } => {
                    println!("{}", text);
                }
                ContentItem::Blob { blob: _ } => {
                    println!("[Binary content skipped]"); // Or handle blobs differently
                }
            }
        }
        println!("---");
    } else {
        println!("\n✕ Resource content was empty.");
    }

    client.close().await?;
    println!("\n✓ Example completed successfully");
    Ok(())
}
