use mcp_client_rust::logging::{LogLevel, McpLogger};
use mcp_client_rust::ollama::{OllamaClient, OllamaMessage, OllamaOptions};
use mcp_client_rust::security::SecurityManager;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = McpLogger::new(LogLevel::Info);

    logger.info("Starting MCP LLM Application with Ollama");

    // Check Ollama
    let ollama = OllamaClient::new("http://localhost:11434", "llama3:latest");

    if !ollama.health_check().await? {
        logger.error("Ollama is not running");
        eprintln!("Start Ollama with: ollama serve");
        std::process::exit(1);
    }

    logger.info("✓ Ollama is running");

    // List available models
    match ollama.list_models().await {
        Ok(models) => println!("Available models: {:?}\n", models),
        Err(e) => logger.warn(&format!("Could not list models: {}", e)),
    }

    let mut security = SecurityManager::new();
    let mut messages: Vec<OllamaMessage> = Vec::new();

    println!("=== MCP LLM Application with Ollama ===");
    println!("Type 'exit' to quit\n");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("exit") {
            break;
        }

        if user_input.is_empty() {
            continue;
        }

        messages.push(OllamaMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        // Get response from Ollama
        match ollama
            .send_message(messages.clone(), Some(OllamaOptions::default()))
            .await
        {
            Ok(response) => {
                println!("Assistant: {}\n", response.message.content);
                messages.push(response.message);
            }
            Err(e) => {
                logger.error(&format!("Failed to get response: {}", e));
                messages.pop(); // Remove the failed message
            }
        }
    }

    println!("Goodbye!");
    Ok(())
}