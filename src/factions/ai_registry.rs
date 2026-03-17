use std::collections::HashMap;

use bevy::prelude::*;

use super::{Faction, FactionId, FactionMemory};
use crate::config::AiConfig;
use crate::resources::Stockpile;

#[derive(Resource, Debug, Default)]
pub struct AiRegistry {
    pub faction_memories: HashMap<FactionId, FactionMemory>,
    pub stockpiles: HashMap<FactionId, Stockpile>,
}

impl AiRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_faction(&mut self, faction: &Faction) {
        self.faction_memories
            .insert(faction.id, FactionMemory::new(faction.id));
        self.stockpiles
            .insert(faction.id, Stockpile::new(faction.id));
    }

    pub fn get_memory(&self, faction_id: FactionId) -> Option<&FactionMemory> {
        self.faction_memories.get(&faction_id)
    }

    pub fn get_memory_mut(&mut self, faction_id: FactionId) -> Option<&mut FactionMemory> {
        self.faction_memories.get_mut(&faction_id)
    }

    pub fn get_stockpile(&self, faction_id: FactionId) -> Option<&Stockpile> {
        self.stockpiles.get(&faction_id)
    }

    pub fn get_stockpile_mut(&mut self, faction_id: FactionId) -> Option<&mut Stockpile> {
        self.stockpiles.get_mut(&faction_id)
    }
}