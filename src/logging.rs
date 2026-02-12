use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

pub struct McpLogger {
    level: LogLevel,
    log_file: Option<String>,
}

impl McpLogger {
    pub fn new(level: LogLevel) -> Self {
        McpLogger {
            level,
            log_file: None,
        }
    }

    pub fn with_file(mut self, path: String) -> Self {
        self.log_file = Some(path);
        self
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level >= self.level {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let level_str = match level {
                LogLevel::Debug => "DEBUG",
                LogLevel::Info => "INFO",
                LogLevel::Warn => "WARN",
                LogLevel::Error => "ERROR",
            };

            let log_message = format!("[{}] [{}] {}", timestamp, level_str, message);

            println!("{}", log_message);

            if let Some(ref log_file) = self.log_file {
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_file)
                {
                    let _ = writeln!(file, "{}", log_message);
                }
            }
        }
    }
}