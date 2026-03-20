use std::collections::HashMap;

use bevy::prelude::*;

use crate::factions::FactionId;
use crate::resources::ResourceType;
use crate::world::WorldPos;

#[derive(Debug, Clone)]
pub enum Command {
    MoveTo {
        creep_id: u32,
        target: WorldPos,
    },
    Mine {
        creep_id: u32,
        mine_id: u32,
    },
    Build {
        creep_id: u32,
        building_type: String,
        x: i32,
        y: i32,
    },
    Attack {
        creep_id: u32,
        target_id: u32,
    },
    Transfer {
        creep_id: u32,
        target_id: u32,
        resource: ResourceType,
        amount: u32,
    },
    Withdraw {
        creep_id: u32,
        target_id: u32,
        resource: ResourceType,
        amount: u32,
    },
    Pickup {
        creep_id: u32,
        resource_type: ResourceType,
    },
    Drop {
        creep_id: u32,
        resource: ResourceType,
        amount: u32,
    },
}

#[derive(Resource, Debug, Clone, Default)]
pub struct CommandBuffer {
    pub commands: HashMap<FactionId, Vec<Command>>,
}

impl CommandBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_command(&mut self, faction_id: FactionId, command: Command) {
        self.commands.entry(faction_id).or_default().push(command);
    }

    pub fn get_commands(&self, faction_id: FactionId) -> Option<&Vec<Command>> {
        self.commands.get(&faction_id)
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn clear_faction(&mut self, faction_id: FactionId) {
        self.commands.remove(&faction_id);
    }
}
