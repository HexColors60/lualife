use bevy::prelude::*;
use std::collections::VecDeque;

/// Screen reader state resource
#[derive(Debug, Clone, Resource, Default)]
pub struct ScreenReaderState {
    pub enabled: bool,
    pub queue: VecDeque<ScreenReaderMessage>,
    pub last_announcement: String,
    pub announcement_count: u32,
}

/// Message for screen reader
#[derive(Debug, Clone)]
pub struct ScreenReaderMessage {
    pub text: String,
    pub priority: MessagePriority,
    pub category: MessageCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageCategory {
    General,
    Combat,
    Economy,
    Building,
    Unit,
    UI,
    Alert,
    Tutorial,
}

impl ScreenReaderState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            queue: VecDeque::new(),
            last_announcement: String::new(),
            announcement_count: 0,
        }
    }

    /// Queue a message for screen reader
    pub fn announce(
        &mut self,
        text: impl Into<String>,
        priority: MessagePriority,
        category: MessageCategory,
    ) {
        let message = ScreenReaderMessage {
            text: text.into(),
            priority,
            category,
        };
        self.queue.push_back(message);
        self.announcement_count += 1;
    }

    /// Queue a high priority alert
    pub fn alert(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::High, MessageCategory::Alert);
    }

    /// Queue a normal message
    pub fn say(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::Normal, MessageCategory::General);
    }

    /// Queue a UI element description
    pub fn describe_ui(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::Normal, MessageCategory::UI);
    }

    /// Queue a unit description
    pub fn describe_unit(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::Normal, MessageCategory::Unit);
    }

    /// Queue a building description
    pub fn describe_building(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::Normal, MessageCategory::Building);
    }

    /// Queue a combat message
    pub fn combat_message(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::High, MessageCategory::Combat);
    }

    /// Queue an economy message
    pub fn economy_message(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::Normal, MessageCategory::Economy);
    }

    /// Queue a tutorial message
    pub fn tutorial_message(&mut self, text: impl Into<String>) {
        self.announce(text, MessagePriority::High, MessageCategory::Tutorial);
    }

    /// Get next message to announce
    pub fn pop_next(&mut self) -> Option<ScreenReaderMessage> {
        if let Some(msg) = self.queue.pop_front() {
            self.last_announcement = msg.text.clone();
            Some(msg)
        } else {
            None
        }
    }

    /// Clear all pending messages
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Get pending message count
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}

/// Component for UI elements that should be announced
#[derive(Debug, Clone, Component)]
pub struct AccessibleElement {
    pub label: String,
    pub description: String,
    pub role: AccessibleRole,
    pub live_region: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessibleRole {
    Button,
    Checkbox,
    Slider,
    Text,
    Image,
    List,
    ListItem,
    Menu,
    MenuItem,
    Dialog,
    Alert,
    Status,
    Progress,
    Tab,
    TabPanel,
}

impl AccessibleElement {
    pub fn new(label: impl Into<String>, role: AccessibleRole) -> Self {
        Self {
            label: label.into(),
            description: String::new(),
            role,
            live_region: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn live_region(mut self) -> Self {
        self.live_region = true;
        self
    }
}

/// System to process screen reader announcements
pub fn screen_reader_announce(
    mut state: ResMut<ScreenReaderState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !state.enabled {
        return;
    }

    // Process messages (in a real implementation, this would interface with TTS)
    while let Some(msg) = state.pop_next() {
        // Log the announcement for now
        tracing::info!(
            "[Screen Reader] {}: {}",
            match msg.priority {
                MessagePriority::Low => "LOW",
                MessagePriority::Normal => "NORMAL",
                MessagePriority::High => "HIGH",
                MessagePriority::Critical => "CRITICAL",
            },
            msg.text
        );
    }

    // Keyboard shortcuts for screen reader
    // R: Read current selection
    if keyboard.just_pressed(KeyCode::KeyR) && keyboard.pressed(KeyCode::AltLeft) {
        state.say("Screen reader active. Press Alt+H for help.");
    }

    // H: Help
    if keyboard.just_pressed(KeyCode::KeyH) && keyboard.pressed(KeyCode::AltLeft) {
        state.say("Screen reader commands: Alt+R to read selection, Alt+H for help, Alt+C to read coordinates, Alt+S to stop speech.");
    }
}

/// Trait for entities that can be described for accessibility
pub trait AccessibleDescription {
    fn get_accessibility_label(&self) -> String;
    fn get_accessibility_description(&self) -> String;
}

/// Helper functions for screen reader text
pub mod text_helpers {
    /// Format a number for screen reader
    pub fn format_number(n: i64) -> String {
        if n >= 1_000_000 {
            format!("{:.1} million", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1} thousand", n as f64 / 1_000.0)
        } else {
            n.to_string()
        }
    }

    /// Format a percentage for screen reader
    pub fn format_percentage(value: f32) -> String {
        format!("{:.0} percent", value * 100.0)
    }

    /// Format coordinates for screen reader
    pub fn format_coordinates(x: i32, y: i32) -> String {
        format!("at position {} by {}", x, y)
    }

    /// Format time duration for screen reader
    pub fn format_duration(seconds: f32) -> String {
        if seconds < 60.0 {
            format!("{:.0} seconds", seconds)
        } else if seconds < 3600.0 {
            let minutes = (seconds / 60.0) as i32;
            let secs = (seconds % 60.0) as i32;
            if secs > 0 {
                format!("{} minutes and {} seconds", minutes, secs)
            } else {
                format!("{} minutes", minutes)
            }
        } else {
            let hours = (seconds / 3600.0) as i32;
            let minutes = ((seconds % 3600.0) / 60.0) as i32;
            if minutes > 0 {
                format!("{} hours and {} minutes", hours, minutes)
            } else {
                format!("{} hours", hours)
            }
        }
    }

    /// Describe health for screen reader
    pub fn describe_health(current: u32, max: u32) -> String {
        let percentage = if max > 0 {
            (current as f32 / max as f32 * 100.0) as i32
        } else {
            0
        };

        if percentage >= 100 {
            "at full health".to_string()
        } else if percentage >= 75 {
            format!("at {} percent health", percentage)
        } else if percentage >= 50 {
            format!("at {} percent health, moderately damaged", percentage)
        } else if percentage >= 25 {
            format!("at {} percent health, heavily damaged", percentage)
        } else if percentage > 0 {
            format!("at {} percent health, critically damaged", percentage)
        } else {
            "destroyed".to_string()
        }
    }

    /// Describe resource amount
    pub fn describe_resource(name: &str, amount: u32) -> String {
        format!("{} {}", format_number(amount as i64), name)
    }
}
