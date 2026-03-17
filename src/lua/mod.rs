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

pub struct LuaPlugin;

impl Plugin for LuaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LuaState>()
            .add_systems(Startup, initialize_lua_vms)
            .add_systems(Update, run_lua_ticks);
    }
}

/// Resource tracking Lua initialization state
#[derive(Resource, Default)]
pub struct LuaState {
    pub initialized: bool,
    pub tick_count: u64,
}

/// Non-send resource for Lua VMs
/// This is stored separately because Lua is not Send+Sync
#[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn get(&self, faction_id: crate::factions::FactionId) -> Option<&LuaVm> {
        self.vms.get(&faction_id)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, faction_id: crate::factions::FactionId) -> Option<&mut LuaVm> {
        self.vms.get_mut(&faction_id)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&crate::factions::FactionId, &mut LuaVm)> {
        self.vms.iter_mut()
    }
}

fn initialize_lua_vms(
    world: &mut World,
) {
    let faction_count = {
        let registry = world.resource::<crate::factions::FactionRegistry>();
        registry.count()
    };

    let mut lua_registry = LuaRegistry::new();
    let mut messages: Vec<String> = Vec::new();

    // Get faction info
    let factions: Vec<(crate::factions::FactionId, String, String)> = {
        let registry = world.resource::<crate::factions::FactionRegistry>();
        registry.all().map(|f| (f.id, f.name.clone(), f.ai_path.clone())).collect()
    };

    for (faction_id, name, ai_path) in factions {
        match LuaVm::new(faction_id) {
            Ok(mut vm) => {
                // Try to load main.lua from faction directory
                let main_path = format!("{}/main.lua", ai_path);
                if std::path::Path::new(&main_path).exists() {
                    if let Err(e) = vm.load_file(std::path::Path::new(&main_path)) {
                        messages.push(format!("Failed to load {} Lua: {}", name, e));
                    } else {
                        messages.push(format!("Loaded Lua for {}", name));
                        // Call on_init if it exists
                        let _ = vm.call_init();
                    }
                } else {
                    messages.push(format!("No Lua script for {} (expected {})", name, main_path));
                }
                lua_registry.register(faction_id, vm);
            }
            Err(e) => {
                messages.push(format!("Failed to create VM for {}: {}", name, e));
            }
        }
    }

    // Insert non-send resource
    world.insert_non_send_resource(lua_registry);

    // Update game log
    let mut game_log = world.resource_mut::<crate::ui::GameLog>();
    for msg in messages {
        game_log.add(msg);
    }
    game_log.add(format!("Lua VMs initialized for {} factions", faction_count));

    // Mark as initialized
    let mut lua_state = world.resource_mut::<LuaState>();
    lua_state.initialized = true;
}

fn run_lua_ticks(
    mut lua_registry: NonSendMut<LuaRegistry>,
    mut lua_state: ResMut<LuaState>,
    game_state: Res<crate::core::GameState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Only run if game is running
    if *game_state != crate::core::GameState::Running {
        return;
    }

    lua_state.tick_count += 1;

    // Run on_tick for each faction's Lua VM
    for (faction_id, vm) in lua_registry.iter_mut() {
        if let Err(e) = vm.call_tick() {
            game_log.add(format!("Lua error for faction {:?}: {}", faction_id, e));
        }
    }
}