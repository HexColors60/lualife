use std::collections::HashMap;

use crate::factions::FactionId;

#[derive(Debug, Clone, Default)]
pub struct FactionMemory {
    pub faction_id: FactionId,
    pub data: HashMap<String, serde_json::Value>,
}

impl FactionMemory {
    pub fn new(faction_id: FactionId) -> Self {
        Self {
            faction_id,
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnitMemory {
    pub creep_id: u32,
    pub data: HashMap<String, serde_json::Value>,
}

impl UnitMemory {
    pub fn new(creep_id: u32) -> Self {
        Self {
            creep_id,
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }
}
