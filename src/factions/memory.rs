use std::collections::HashMap;

use bevy::prelude::*;

use super::FactionId;

#[derive(Debug, Clone, Default)]
pub struct FactionMemory {
    pub faction_id: FactionId,
    pub data: HashMap<String, String>,
    pub room_memory: HashMap<(u32, u32), HashMap<String, String>>,
}

impl FactionMemory {
    pub fn new(faction_id: FactionId) -> Self {
        Self {
            faction_id,
            data: HashMap::new(),
            room_memory: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn get_room_memory(&self, room_x: u32, room_y: u32, key: &str) -> Option<&String> {
        self.room_memory.get(&(room_x, room_y))?.get(key)
    }

    pub fn set_room_memory(&mut self, room_x: u32, room_y: u32, key: String, value: String) {
        self.room_memory
            .entry((room_x, room_y))
            .or_default()
            .insert(key, value);
    }
}