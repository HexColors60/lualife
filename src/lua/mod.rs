mod api;
mod commands;
mod hooks;
mod hot_reload;
mod loader;
mod memory;
mod sandbox;
mod vm;

pub use api::*;
pub use commands::*;
pub use hooks::*;
pub use hot_reload::*;
pub use loader::*;
pub use memory::*;
pub use sandbox::*;
pub use vm::*;

use bevy::prelude::*;

use crate::config::AiConfig;

pub struct LuaPlugin;

impl Plugin for LuaPlugin {
    fn build(&self, _app: &mut App) {
        // LuaRegistry is not a Resource because Lua is not Send+Sync
        // It will be managed separately
    }
}

/// Non-resource registry for Lua VMs
pub struct LuaRegistry {
    pub vms: std::collections::HashMap<crate::factions::FactionId, LuaVm>,
}

impl Default for LuaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LuaRegistry {
    pub fn new() -> Self {
        Self {
            vms: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, faction_id: crate::factions::FactionId, vm: LuaVm) {
        self.vms.insert(faction_id, vm);
    }

    pub fn get(&self, faction_id: crate::factions::FactionId) -> Option<&LuaVm> {
        self.vms.get(&faction_id)
    }

    pub fn get_mut(&mut self, faction_id: crate::factions::FactionId) -> Option<&mut LuaVm> {
        self.vms.get_mut(&faction_id)
    }
}