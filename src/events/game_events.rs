use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
pub struct EventLog {
    pub entries: VecDeque<LogEntry>,
    pub max_entries: usize,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries: 1000,
        }
    }

    pub fn log(&mut self, message: String, level: LogLevel) {
        let entry = LogEntry {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            level,
            message,
        };

        self.entries.push_back(entry);

        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn info(&mut self, message: String) {
        self.log(message, LogLevel::Info);
    }

    pub fn warn(&mut self, message: String) {
        self.log(message, LogLevel::Warn);
    }

    pub fn error(&mut self, message: String) {
        self.log(message, LogLevel::Error);
    }

    pub fn debug(&mut self, message: String) {
        self.log(message, LogLevel::Debug);
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
