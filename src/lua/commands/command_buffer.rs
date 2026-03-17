use std::collections::HashMap;

use crate::factions::FactionId;
use crate::sim::Command;

pub struct FactionCommandBuffer {
    pub commands: HashMap<FactionId, Vec<Command>>,
}

impl Default for FactionCommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl FactionCommandBuffer {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn add(&mut self, faction_id: FactionId, command: Command) {
        self.commands.entry(faction_id).or_default().push(command);
    }

    pub fn get(&self, faction_id: FactionId) -> Option<&Vec<Command>> {
        self.commands.get(&faction_id)
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
