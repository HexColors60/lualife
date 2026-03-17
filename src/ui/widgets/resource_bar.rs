use std::collections::HashMap;

use crate::resources::ResourceType;

/// Resource bar widget for displaying faction resources
pub struct ResourceBar {
    pub resources: HashMap<ResourceType, (u32, u32)>, // (current, max)
}

impl Default for ResourceBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceBar {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn set(&mut self, resource: ResourceType, current: u32, max: u32) {
        self.resources.insert(resource, (current, max));
    }

    pub fn get(&self, resource: ResourceType) -> (u32, u32) {
        self.resources.get(&resource).copied().unwrap_or((0, 0))
    }

    pub fn fill_ratio(&self, resource: ResourceType) -> f32 {
        let (current, max) = self.get(resource);
        if max == 0 {
            return 0.0;
        }
        current as f32 / max as f32
    }
}