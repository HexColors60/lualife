use std::collections::HashMap;
use std::net::SocketAddr;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;
use crate::core::TickNumber;

/// Network mode: single player, server, or client
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetworkMode {
    #[default]
    SinglePlayer,
    Server,
    Client,
}

/// Network configuration
#[derive(Resource, Debug, Clone)]
pub struct NetworkConfig {
    pub mode: NetworkMode,
    pub server_address: SocketAddr,
    pub max_players: usize,
    pub tick_rate: u32, // ticks per second
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            mode: NetworkMode::SinglePlayer,
            server_address: "127.0.0.1:7777".parse().unwrap(),
            max_players: 32,
            tick_rate: 20,
        }
    }
}

/// Network connection state
#[derive(Resource, Debug, Clone, Default)]
pub struct NetworkState {
    pub connected: bool,
    pub client_id: Option<u64>,
    pub players: HashMap<u64, PlayerInfo>,
    pub ping_ms: u32,
}

/// Player information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: u64,
    pub name: String,
    pub faction_id: Option<FactionId>,
    pub ready: bool,
}

/// Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // Connection
    Connect { name: String },
    Disconnect { client_id: u64 },
    Connected { client_id: u64, tick: TickNumber },
    
    // Game state
    GameStateSync { tick: TickNumber, data: Vec<u8> },
    TickUpdate { tick: TickNumber, commands: Vec<ClientCommand> },
    
    // Chat
    ChatMessage { client_id: u64, message: String },
    
    // Player management
    PlayerJoined { player: PlayerInfo },
    PlayerLeft { client_id: u64 },
    PlayerReady { client_id: u64, ready: bool },
    
    // Errors
    Error { message: String },
}

/// Command from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCommand {
    pub client_id: u64,
    pub tick: TickNumber,
    pub command_type: CommandType,
}

/// Types of commands clients can send
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    MoveCreep { creep_id: u32, target_x: i32, target_y: i32 },
    AttackCreep { attacker_id: u32, target_id: u32 },
    BuildStructure { building_type: u8, x: i32, y: i32 },
    SpawnCreep { body_parts: Vec<u8> },
    SendMessage { message: String },
}

/// Event for network messages
#[derive(Event, Debug, Clone)]
pub struct NetworkEvent {
    pub message: NetworkMessage,
}

/// System to process network events
pub fn network_event_system(
    mut events: EventReader<NetworkEvent>,
    mut network_state: ResMut<NetworkState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for event in events.read() {
        match &event.message {
            NetworkMessage::Connected { client_id, tick } => {
                network_state.connected = true;
                network_state.client_id = Some(*client_id);
                game_log.add(format!("Connected to server as client {}", client_id));
            }
            NetworkMessage::PlayerJoined { player } => {
                network_state.players.insert(player.id, player.clone());
                game_log.add(format!("Player {} joined", player.name));
            }
            NetworkMessage::PlayerLeft { client_id } => {
                if let Some(player) = network_state.players.remove(client_id) {
                    game_log.add(format!("Player {} left", player.name));
                }
            }
            NetworkMessage::ChatMessage { client_id, message } => {
                if let Some(player) = network_state.players.get(client_id) {
                    game_log.add(format!("{}: {}", player.name, message));
                }
            }
            NetworkMessage::Error { message } => {
                game_log.add(format!("Network error: {}", message));
            }
            _ => {}
        }
    }
}

/// Plugin for network system
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkConfig>()
            .init_resource::<NetworkState>()
            .add_event::<NetworkEvent>()
            .add_systems(Update, network_event_system);
    }
}