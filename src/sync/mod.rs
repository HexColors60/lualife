use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::TickNumber;
use crate::network::{ClientCommand, CommandType};

/// Synchronization state
#[derive(Resource, Debug, Clone, Default)]
pub struct SyncState {
    pub server_tick: TickNumber,
    pub client_tick: TickNumber,
    pub tick_buffer: Vec<TickData>,
    pub pending_commands: HashMap<TickNumber, Vec<ClientCommand>>,
    pub last_acked_tick: TickNumber,
    pub rollbacks: u32,
}

/// Tick data for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickData {
    pub tick: TickNumber,
    pub checksum: u64,
    pub commands: Vec<ClientCommand>,
}

impl SyncState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_command(&mut self, command: ClientCommand) {
        let tick = command.tick;
        self.pending_commands.entry(tick).or_default().push(command);
    }

    pub fn get_commands(&mut self, tick: TickNumber) -> Vec<ClientCommand> {
        self.pending_commands.remove(&tick).unwrap_or_default()
    }

    pub fn advance_tick(&mut self) {
        self.client_tick.0 += 1;
    }

    pub fn sync_to_server(&mut self, server_tick: TickNumber) {
        if server_tick.0 > self.server_tick.0 {
            self.server_tick = server_tick;
        }
    }

    pub fn is_behind(&self) -> bool {
        self.client_tick.0 < self.server_tick.0
    }

    pub fn tick_delta(&self) -> i64 {
        self.server_tick.0 as i64 - self.client_tick.0 as i64
    }
}

/// Game state checksum for verification
#[derive(Debug, Clone, Copy, Default)]
pub struct StateChecksum {
    pub creep_checksum: u64,
    pub building_checksum: u64,
    pub resource_checksum: u64,
}

impl StateChecksum {
    pub fn combined(&self) -> u64 {
        self.creep_checksum
            .wrapping_mul(31)
            .wrapping_add(self.building_checksum)
            .wrapping_mul(31)
            .wrapping_add(self.resource_checksum)
    }
}

/// Deterministic random seed for synchronized simulation
#[derive(Resource, Debug, Clone)]
pub struct SyncRandom {
    pub seed: u64,
    pub state: u64,
}

impl SyncRandom {
    pub fn new(seed: u64) -> Self {
        Self { seed, state: seed }
    }

    /// Deterministic random number generator (xorshift)
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_range(&mut self, min: u32, max: u32) -> u32 {
        if max <= min {
            return min;
        }
        min + (self.next_u32() % (max - min))
    }

    pub fn reset(&mut self) {
        self.state = self.seed;
    }
}

/// Lockstep configuration
#[derive(Resource, Debug, Clone)]
pub struct LockstepConfig {
    pub enabled: bool,
    pub max_commands_per_tick: usize,
    pub timeout_ms: u32,
    pub rollback_limit: u32,
}

impl Default for LockstepConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_commands_per_tick: 1000,
            timeout_ms: 5000,
            rollback_limit: 10,
        }
    }
}

/// System to synchronize game state
pub fn sync_system(mut sync_state: ResMut<SyncState>, tick: Res<TickNumber>) {
    if tick.is_changed() {
        sync_state.advance_tick();
    }
}

/// System to process pending commands
pub fn command_sync_system(mut sync_state: ResMut<SyncState>, tick: Res<TickNumber>) {
    // Process commands for current tick
    let _commands = sync_state.get_commands(*tick);
    // Commands would be applied to game state here
}

/// Plugin for synchronization system
pub struct SyncPlugin;

impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SyncState>()
            .init_resource::<LockstepConfig>()
            .add_systems(Update, (sync_system, command_sync_system));
    }
}
