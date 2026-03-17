use std::collections::VecDeque;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender_id: u64,
    pub sender_name: String,
    pub content: String,
    pub channel: ChatChannel,
    pub timestamp: u64,
}

/// Chat channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ChatChannel {
    #[default]
    Global,      // Everyone
    Team,        // Team/faction members only
    Whisper,     // Private message
    System,      // System messages
}

/// Chat history
#[derive(Resource, Debug, Clone, Default)]
pub struct ChatHistory {
    pub messages: VecDeque<ChatMessage>,
    pub max_messages: usize,
}

impl ChatHistory {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 100,
        }
    }

    pub fn add(&mut self, message: ChatMessage) {
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }

    pub fn get_recent(&self, count: usize) -> Vec<&ChatMessage> {
        self.messages.iter().rev().take(count).rev().collect()
    }

    pub fn get_by_channel(&self, channel: ChatChannel) -> Vec<&ChatMessage> {
        self.messages.iter().filter(|m| m.channel == channel).collect()
    }
}

/// Chat input state
#[derive(Resource, Debug, Clone, Default)]
pub struct ChatInput {
    pub active: bool,
    pub input: String,
    pub target_channel: ChatChannel,
    pub whisper_target: Option<u64>,
}

impl ChatInput {
    pub fn new() -> Self {
        Self {
            active: false,
            input: String::new(),
            target_channel: ChatChannel::Global,
            whisper_target: None,
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if !self.active {
            self.input.clear();
        }
    }

    pub fn set_channel(&mut self, channel: ChatChannel) {
        self.target_channel = channel;
    }

    pub fn add_char(&mut self, c: char) {
        if self.active {
            self.input.push(c);
        }
    }

    pub fn remove_char(&mut self) {
        if self.active && !self.input.is_empty() {
            self.input.pop();
        }
    }

    pub fn submit(&mut self) -> Option<String> {
        if self.active && !self.input.is_empty() {
            let message = self.input.clone();
            self.input.clear();
            Some(message)
        } else {
            None
        }
    }
}

/// Chat commands
#[derive(Debug, Clone)]
pub enum ChatCommand {
    Global(String),
    Team(String),
    Whisper(u64, String),
    Help,
    Players,
    Time,
}

impl ChatCommand {
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        
        if input.starts_with("/g ") {
            Some(Self::Global(input[3..].to_string()))
        } else if input.starts_with("/t ") || input.starts_with("/team ") {
            let msg = if input.starts_with("/t ") { &input[3..] } else { &input[6..] };
            Some(Self::Team(msg.to_string()))
        } else if input.starts_with("/w ") || input.starts_with("/whisper ") {
            let rest = if input.starts_with("/w ") { &input[3..] } else { &input[9..] };
            let parts: Vec<&str> = rest.splitn(2, ' ').collect();
            if parts.len() == 2 {
                if let Ok(target_id) = parts[0].parse::<u64>() {
                    Some(Self::Whisper(target_id, parts[1].to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        } else if input == "/help" {
            Some(Self::Help)
        } else if input == "/players" {
            Some(Self::Players)
        } else if input == "/time" {
            Some(Self::Time)
        } else if !input.is_empty() {
            Some(Self::Global(input.to_string()))
        } else {
            None
        }
    }
}

/// Chat events
#[derive(Event, Debug, Clone)]
pub enum ChatEvent {
    SendMessage { sender_id: u64, sender_name: String, content: String, channel: ChatChannel },
    ReceiveMessage { message: ChatMessage },
}

/// System to process chat events
pub fn chat_event_system(
    mut events: EventReader<ChatEvent>,
    mut chat_history: ResMut<ChatHistory>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for event in events.read() {
        match event {
            ChatEvent::ReceiveMessage { message } => {
                chat_history.add(message.clone());
                
                let channel_prefix = match message.channel {
                    ChatChannel::Global => "[Global]",
                    ChatChannel::Team => "[Team]",
                    ChatChannel::Whisper => "[Whisper]",
                    ChatChannel::System => "[System]",
                };
                
                game_log.add(format!("{} {}: {}", channel_prefix, message.sender_name, message.content));
            }
            _ => {}
        }
    }
}

/// System to toggle chat input
pub fn chat_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut chat_input: ResMut<ChatInput>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        chat_input.toggle();
    }
}

/// Plugin for chat system
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChatHistory>()
            .init_resource::<ChatInput>()
            .add_event::<ChatEvent>()
            .add_systems(Update, (
                chat_event_system,
                chat_input_system,
            ));
    }
}