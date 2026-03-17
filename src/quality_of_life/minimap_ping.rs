use bevy::prelude::*;
use std::collections::VecDeque;

/// Minimap ping state resource
#[derive(Debug, Clone, Resource)]
pub struct MinimapPingState {
    pub pings: VecDeque<MinimapPing>,
    pub drawings: Vec<MinimapDrawing>,
    pub drawing_enabled: bool,
    pub current_color: PingColor,
    pub max_pings: usize,
    pub ping_duration: f32,
}

impl Default for MinimapPingState {
    fn default() -> Self {
        Self {
            pings: VecDeque::new(),
            drawings: Vec::new(),
            drawing_enabled: false,
            current_color: PingColor::Yellow,
            max_pings: 20,
            ping_duration: 5.0,
        }
    }
}

/// Minimap ping
#[derive(Debug, Clone)]
pub struct MinimapPing {
    pub position: (f32, f32),
    pub ping_type: PingType,
    pub color: PingColor,
    pub created_at: f32,
    pub duration: f32,
    pub faction_id: Option<u16>,
}

/// Ping type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PingType {
    Alert,      // Attention ping
    Attack,     // Attack here
    Defend,     // Defend here
    Gather,     // Gather here
    Danger,     // Danger zone
    Custom,     // Custom ping
}

/// Ping color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PingColor {
    Yellow,
    Red,
    Green,
    Blue,
    Purple,
    White,
}

impl PingColor {
    pub fn to_color(&self) -> Color {
        match self {
            PingColor::Yellow => Color::srgb(1.0, 1.0, 0.0),
            PingColor::Red => Color::srgb(1.0, 0.0, 0.0),
            PingColor::Green => Color::srgb(0.0, 1.0, 0.0),
            PingColor::Blue => Color::srgb(0.0, 0.5, 1.0),
            PingColor::Purple => Color::srgb(0.8, 0.0, 0.8),
            PingColor::White => Color::srgb(1.0, 1.0, 1.0),
        }
    }
}

/// Minimap drawing
#[derive(Debug, Clone)]
pub struct MinimapDrawing {
    pub points: Vec<(f32, f32)>,
    pub color: PingColor,
    pub created_at: f32,
    pub duration: f32,
}

/// Minimap ping event
#[derive(Debug, Clone, Event)]
pub enum MinimapPingEvent {
    Ping(MinimapPing),
    ClearPings,
    StartDrawing,
    StopDrawing,
    AddDrawPoint(f32, f32),
    ClearDrawings,
    SetColor(PingColor),
}

impl MinimapPingState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a ping
    pub fn add_ping(&mut self, ping: MinimapPing) {
        if self.pings.len() >= self.max_pings {
            self.pings.pop_front();
        }
        self.pings.push_back(ping);
    }

    /// Remove expired pings
    pub fn remove_expired(&mut self, current_time: f32) {
        self.pings.retain(|p| current_time - p.created_at < p.duration);
        self.drawings.retain(|d| current_time - d.created_at < d.duration);
    }

    /// Clear all pings
    pub fn clear_pings(&mut self) {
        self.pings.clear();
    }

    /// Clear all drawings
    pub fn clear_drawings(&mut self) {
        self.drawings.clear();
    }

    /// Start a new drawing
    pub fn start_drawing(&mut self) {
        self.drawing_enabled = true;
        self.drawings.push(MinimapDrawing {
            points: Vec::new(),
            color: self.current_color,
            created_at: 0.0,
            duration: 30.0,
        });
    }

    /// Stop drawing
    pub fn stop_drawing(&mut self) {
        self.drawing_enabled = false;
    }

    /// Add a point to the current drawing
    pub fn add_draw_point(&mut self, x: f32, y: f32) {
        if let Some(drawing) = self.drawings.last_mut() {
            drawing.points.push((x, y));
        }
    }

    /// Set the current ping color
    pub fn set_color(&mut self, color: PingColor) {
        self.current_color = color;
    }

    /// Get active pings
    pub fn get_active_pings(&self, current_time: f32) -> Vec<&MinimapPing> {
        self.pings.iter()
            .filter(|p| current_time - p.created_at < p.duration)
            .collect()
    }
}

/// System to handle minimap pings
pub fn minimap_ping_system(
    mut state: ResMut<MinimapPingState>,
    mut events: EventReader<MinimapPingEvent>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let current_time = time.elapsed_seconds();

    // Remove expired pings
    state.remove_expired(current_time);

    for event in events.read() {
        match event {
            MinimapPingEvent::Ping(ping) => {
                state.add_ping(ping.clone());
                tracing::info!("Ping at {:?}", ping.position);
            }
            MinimapPingEvent::ClearPings => {
                state.clear_pings();
            }
            MinimapPingEvent::StartDrawing => {
                state.start_drawing();
            }
            MinimapPingEvent::StopDrawing => {
                state.stop_drawing();
            }
            MinimapPingEvent::AddDrawPoint(x, y) => {
                state.add_draw_point(*x, *y);
            }
            MinimapPingEvent::ClearDrawings => {
                state.clear_drawings();
            }
            MinimapPingEvent::SetColor(color) => {
                state.set_color(*color);
            }
        }
    }

    // Keyboard shortcuts for ping types
    let alt = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);

    if alt {
        if keyboard.just_pressed(KeyCode::KeyA) {
            // Alt+A: Attack ping
            let duration = state.ping_duration;
            state.add_ping(MinimapPing {
                position: (0.0, 0.0), // Would be mouse position
                ping_type: PingType::Attack,
                color: PingColor::Red,
                created_at: current_time,
                duration,
                faction_id: None,
            });
        }
        if keyboard.just_pressed(KeyCode::KeyD) {
            // Alt+D: Defend ping
            let duration = state.ping_duration;
            state.add_ping(MinimapPing {
                position: (0.0, 0.0),
                ping_type: PingType::Defend,
                color: PingColor::Green,
                created_at: current_time,
                duration,
                faction_id: None,
            });
        }
        if keyboard.just_pressed(KeyCode::KeyG) {
            // Alt+G: Gather ping
            let duration = state.ping_duration;
            state.add_ping(MinimapPing {
                position: (0.0, 0.0),
                ping_type: PingType::Gather,
                color: PingColor::Yellow,
                created_at: current_time,
                duration,
                faction_id: None,
            });
        }
    }
}

/// Create a ping at a position
pub fn create_ping(
    position: (f32, f32),
    ping_type: PingType,
    color: PingColor,
    faction_id: Option<u16>,
) -> MinimapPing {
    MinimapPing {
        position,
        ping_type,
        color,
        created_at: 0.0,
        duration: 5.0,
        faction_id,
    }
}