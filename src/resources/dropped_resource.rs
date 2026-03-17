use bevy::prelude::*;

use super::ResourceType;

#[derive(Debug, Clone, Component)]
pub struct DroppedResource {
    pub resource_type: ResourceType,
    pub amount: u32,
}

impl DroppedResource {
    pub fn new(resource_type: ResourceType, amount: u32) -> Self {
        Self {
            resource_type,
            amount,
        }
    }
}
