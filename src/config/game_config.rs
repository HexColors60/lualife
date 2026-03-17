use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::consts::*;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Ticks per second
    pub tick_rate: u64,

    /// Whether GOD mode is enabled by default
    pub god_mode: bool,

    /// Number of AI factions
    pub ai_count: usize,

    /// Autosave interval in ticks (0 = disabled)
    pub autosave_interval: u64,

    /// Simulation speed multiplier
    pub sim_speed: f32,

    /// Lua instruction budget per tick per faction
    pub lua_budget: usize,

    /// Whether to start paused
    pub start_paused: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tick_rate: DEFAULT_TICK_RATE,
            god_mode: true,
            ai_count: DEFAULT_AI_COUNT,
            autosave_interval: 0,
            sim_speed: 1.0,
            lua_budget: DEFAULT_LUA_BUDGET,
            start_paused: false,
        }
    }
}
