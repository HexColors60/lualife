use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::resources::ResourceType;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub resources: HashMap<ResourceType, u32>,
    pub capacity: u32,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            resources: HashMap::new(),
            capacity,
        }
    }

    pub fn get(&self, resource_type: ResourceType) -> u32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }

    pub fn total(&self) -> u32 {
        self.resources.values().sum()
    }

    pub fn available_capacity(&self) -> u32 {
        self.capacity.saturating_sub(self.total())
    }

    pub fn is_full(&self) -> bool {
        self.total() >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    pub fn add(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let space = self.available_capacity();
        let to_add = amount.min(space);
        let current = self.get(resource_type);
        self.resources.insert(resource_type, current + to_add);
        to_add
    }

    pub fn remove(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let current = self.get(resource_type);
        let to_remove = amount.min(current);
        let remaining = current - to_remove;

        if remaining == 0 {
            self.resources.remove(&resource_type);
        } else {
            self.resources.insert(resource_type, remaining);
        }

        to_remove
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }
}
