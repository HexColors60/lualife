use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::ResourceType;
use crate::factions::FactionId;

#[derive(Resource, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stockpile {
    pub faction_id: FactionId,
    pub resources: HashMap<ResourceType, u32>,
    pub capacity: HashMap<ResourceType, u32>,
}

impl Stockpile {
    pub fn new(faction_id: FactionId) -> Self {
        Self {
            faction_id,
            resources: HashMap::new(),
            capacity: HashMap::new(),
        }
    }

    pub fn get(&self, resource_type: ResourceType) -> u32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }

    pub fn set(&mut self, resource_type: ResourceType, amount: u32) {
        self.resources.insert(resource_type, amount);
    }

    pub fn add(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let current = self.get(resource_type);
        let new_amount = current.saturating_add(amount);
        self.set(resource_type, new_amount);
        new_amount
    }

    pub fn remove(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let current = self.get(resource_type);
        let removed = amount.min(current);
        let new_amount = current - removed;
        self.set(resource_type, new_amount);
        removed
    }

    pub fn has(&self, resource_type: ResourceType, amount: u32) -> bool {
        self.get(resource_type) >= amount
    }

    pub fn total(&self) -> u32 {
        self.resources.values().sum()
    }

    pub fn set_capacity(&mut self, resource_type: ResourceType, capacity: u32) {
        self.capacity.insert(resource_type, capacity);
    }

    pub fn get_capacity(&self, resource_type: ResourceType) -> u32 {
        self.capacity
            .get(&resource_type)
            .copied()
            .unwrap_or(u32::MAX)
    }

    pub fn can_add(&self, resource_type: ResourceType, amount: u32) -> bool {
        let current = self.get(resource_type);
        let capacity = self.get_capacity(resource_type);
        current.saturating_add(amount) <= capacity
    }
}
