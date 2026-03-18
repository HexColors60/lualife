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

    pub fn pack_all(&mut self) -> HashMap<ResourceType, u32> {
        std::mem::take(&mut self.resources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_new() {
        let inv = Inventory::new(100);
        assert_eq!(inv.capacity, 100);
        assert!(inv.is_empty());
    }

    #[test]
    fn test_inventory_add() {
        let mut inv = Inventory::new(100);
        let added = inv.add(ResourceType::Iron, 50);
        assert_eq!(added, 50);
        assert_eq!(inv.get(ResourceType::Iron), 50);
        assert_eq!(inv.total(), 50);
    }

    #[test]
    fn test_inventory_add_over_capacity() {
        let mut inv = Inventory::new(100);
        let added = inv.add(ResourceType::Iron, 150);
        assert_eq!(added, 100);
        assert_eq!(inv.get(ResourceType::Iron), 100);
        assert_eq!(inv.total(), 100);
    }

    #[test]
    fn test_inventory_remove() {
        let mut inv = Inventory::new(100);
        inv.add(ResourceType::Iron, 50);
        let removed = inv.remove(ResourceType::Iron, 30);
        assert_eq!(removed, 30);
        assert_eq!(inv.get(ResourceType::Iron), 20);
    }

    #[test]
    fn test_inventory_remove_more_than_exists() {
        let mut inv = Inventory::new(100);
        inv.add(ResourceType::Iron, 50);
        let removed = inv.remove(ResourceType::Iron, 100);
        assert_eq!(removed, 50);
        assert_eq!(inv.get(ResourceType::Iron), 0);
    }

    #[test]
    fn test_inventory_multiple_resources() {
        let mut inv = Inventory::new(100);
        inv.add(ResourceType::Iron, 30);
        inv.add(ResourceType::Copper, 40);
        assert_eq!(inv.total(), 70);
        assert_eq!(inv.get(ResourceType::Iron), 30);
        assert_eq!(inv.get(ResourceType::Copper), 40);
    }

    #[test]
    fn test_inventory_is_full() {
        let mut inv = Inventory::new(100);
        assert!(!inv.is_full());
        inv.add(ResourceType::Iron, 100);
        assert!(inv.is_full());
    }

    #[test]
    fn test_inventory_available_capacity() {
        let mut inv = Inventory::new(100);
        assert_eq!(inv.available_capacity(), 100);
        inv.add(ResourceType::Iron, 30);
        assert_eq!(inv.available_capacity(), 70);
    }

    #[test]
    fn test_inventory_clear() {
        let mut inv = Inventory::new(100);
        inv.add(ResourceType::Iron, 50);
        inv.add(ResourceType::Copper, 30);
        inv.clear();
        assert!(inv.is_empty());
        assert_eq!(inv.total(), 0);
    }

    #[test]
    fn test_inventory_get_missing_resource() {
        let inv = Inventory::new(100);
        assert_eq!(inv.get(ResourceType::Iron), 0);
    }

    #[test]
    fn test_inventory_pack_all() {
        let mut inv = Inventory::new(100);
        inv.add(ResourceType::Iron, 30);
        inv.add(ResourceType::Copper, 20);
        let packed = inv.pack_all();
        assert!(inv.is_empty());
        assert_eq!(packed.get(&ResourceType::Iron), Some(&30));
        assert_eq!(packed.get(&ResourceType::Copper), Some(&20));
    }
}
