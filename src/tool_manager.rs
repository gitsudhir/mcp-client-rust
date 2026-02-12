use crate::types::Tool;
use serde_json::{json, Value};

pub struct ToolManager {
    tools: Vec<Tool>,
}

impl ToolManager {
    pub fn new(tools: Vec<Tool>) -> Self {
        ToolManager { tools }
    }

    pub fn format_for_llm(&self) -> Vec<Value> {
        self.tools
            .iter()
            .map(|tool| {
                json!({
                    "name": tool.name,
                    "description": tool.description.as_deref().unwrap_or(&format!("Tool: {}", tool.name)),
                    "parameters": tool.input_schema
                })
            })
            .collect()
    }

    pub fn find_tool(&self, name: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.name == name)
    }

    pub fn get_all_tools(&self) -> &[Tool] {
        &self.tools
    }
}