use mcp_client_rust::logging::{LogLevel, McpLogger};
use mcp_client_rust::ollama::{OllamaClient, OllamaMessage, OllamaOptions};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = McpLogger::new(LogLevel::Info);

    // Initialize Ollama client
    let ollama = OllamaClient::new("http://localhost:11434", "llama3:latest");

    // Check if Ollama is available
    if !ollama.health_check().await? {
        logger.error("Ollama is not running");
        eprintln!("Please start Ollama first: ollama serve");
        eprintln!("Download from: https://ollama.ai");
        std::process::exit(1);
    }

    println!("✓ Ollama is running");

    // List available models
    match ollama.list_models().await {
        Ok(models) => println!("Available models: {:?}", models),
        Err(e) => logger.warn(&format!("Could not list models: {}", e)),
    }

    // Initialize MCP client (example - would need actual server)
    println!("\n=== MCP Client Initialization ===");
    println!("Note: This example requires a running MCP server");
    println!("Tools would be discovered and displayed here");

    // Example tool definitions (mock data)
    let sample_response = r#"{
        "role": "assistant",
        "content": "I have access to various tools that I can use to help you. In a real scenario, these would be discovered from the MCP server."
    }"#;

    println!("\nOllama Response:");
    println!("{}", sample_response);

    let messages = vec![OllamaMessage {
        role: "user".to_string(),
        content: "What tools are available?".to_string(),
    }];

    match ollama.send_message(messages, Some(OllamaOptions::default())).await {
        Ok(response) => {
            println!("\n=== Ollama Response ===");
            println!("Model: {}", response.model);
            println!("Message: {}", response.message.content);
            println!("Done: {}", response.done);
        }
        Err(e) => {
            logger.error(&format!("Failed to get response: {}", e));
        }
    }

    Ok(())
}