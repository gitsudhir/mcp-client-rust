use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ToolPolicy {
    pub requires_approval: bool,
    pub max_calls_per_minute: usize,
}

pub struct SecurityManager {
    tool_policies: HashMap<String, ToolPolicy>,
    call_history: HashMap<String, Vec<Instant>>,
}

impl SecurityManager {
    pub fn new() -> Self {
        let mut policies = HashMap::new();

        policies.insert(
            "file-write".to_string(),
            ToolPolicy {
                requires_approval: true,
                max_calls_per_minute: 5,
            },
        );

        policies.insert(
            "execute-command".to_string(),
            ToolPolicy {
                requires_approval: true,
                max_calls_per_minute: 2,
            },
        );

        policies.insert(
            "read-data".to_string(),
            ToolPolicy {
                requires_approval: false,
                max_calls_per_minute: 20,
            },
        );

        SecurityManager {
            tool_policies: policies,
            call_history: HashMap::new(),
        }
    }

    pub fn set_tool_policy(&mut self, tool_name: String, policy: ToolPolicy) {
        self.tool_policies.insert(tool_name, policy);
    }

    pub fn check_tool_call(&mut self, tool_name: &str) -> bool {
        let policy = self
            .tool_policies
            .get(tool_name)
            .cloned()
            .unwrap_or(ToolPolicy {
                requires_approval: true,
                max_calls_per_minute: 10,
            });

        self.check_rate_limit(tool_name, policy.max_calls_per_minute)
    }

    fn check_rate_limit(&mut self, tool_name: &str, max_calls: usize) -> bool {
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);

        let history = self
            .call_history
            .entry(tool_name.to_string())
            .or_insert_with(Vec::new);

        history.retain(|&call_time| call_time > one_minute_ago);

        if history.len() >= max_calls {
            return false;
        }

        history.push(now);
        true
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}