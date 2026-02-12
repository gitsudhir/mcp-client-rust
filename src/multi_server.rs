use crate::client::MCPClient;
use crate::transport::Transport;
use crate::types::*;
use crate::errors::McpResult;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
}

pub struct MultiServerManager {
    servers: HashMap<String, MCPClient>,
    server_configs: HashMap<String, ServerConfig>,
}

impl MultiServerManager {
    pub fn new() -> Self {
        MultiServerManager {
            servers: HashMap::new(),
            server_configs: HashMap::new(),
        }
    }

    pub async fn add_server(
        &mut self,
        transport: Box<dyn Transport>,
        config: ServerConfig,
    ) -> McpResult<()> {
        let client_info = ClientInfo {
            name: "MultiServerClient".to_string(),
            version: "1.0.0".to_string(),
        };

        let mut client = MCPClient::new(transport.into(), client_info);
        client.initialize().await?;

        self.server_configs.insert(config.id.clone(), config.clone());
        self.servers.insert(config.id.clone(), client);

        Ok(())
    }

    pub async fn remove_server(&mut self, id: &str) -> McpResult<()> {
        if let Some(mut client) = self.servers.remove(id) {
            client.close().await?;
        }
        self.server_configs.remove(id);
        Ok(())
    }

    pub fn get_client(&mut self, id: &str) -> Option<&mut MCPClient> {
        self.servers.get_mut(id)
    }

    pub async fn get_all_tools(&mut self) -> McpResult<Vec<(String, Tool)>> {
        let mut all_tools = Vec::new();
        let server_ids: Vec<_> = self.servers.keys().cloned().collect();

        for server_id in server_ids {
            if let Some(client) = self.servers.get_mut(&server_id) {
                match client.list_tools().await {
                    Ok(tools) => {
                        for tool in tools {
                            all_tools.push((server_id.clone(), tool));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting tools from server {}: {}", server_id, e);
                    }
                }
            }
        }

        Ok(all_tools)
    }

    pub fn get_server_ids(&self) -> Vec<String> {
        self.servers.keys().cloned().collect()
    }
}

impl Default for MultiServerManager {
    fn default() -> Self {
        Self::new()
    }
}
