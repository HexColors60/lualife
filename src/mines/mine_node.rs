use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::MineType;
use crate::resources::ResourceType;
use crate::world::WorldPos;

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct MineNode {
    pub id: u32,
    pub mine_type: MineType,
    pub current_amount: u32,
    pub max_amount: u32,
    pub position: WorldPos,
    pub exhausted: bool,
}

impl MineNode {
    pub fn new(id: u32, resource_type: ResourceType, position: WorldPos, max_amount: u32) -> Self {
        Self {
            id,
            mine_type: MineType::for_resource(resource_type),
            current_amount: max_amount,
            max_amount,
            position,
            exhausted: false,
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        self.mine_type.resource_type
    }

    pub fn can_extract(&self) -> bool {
        !self.exhausted && self.current_amount > 0
    }

    pub fn extract(&mut self, amount: u32) -> u32 {
        if self.exhausted {
            return 0;
        }

        let extracted = amount.min(self.current_amount);
        self.current_amount = self.current_amount.saturating_sub(extracted);

        if self.current_amount == 0 {
            self.exhausted = true;
        }

        extracted
    }

    pub fn regenerate(&mut self) {
        if self.current_amount < self.max_amount {
            let regen = (self.max_amount as f32 * self.mine_type.regen_rate) as u32;
            self.current_amount = (self.current_amount + regen).min(self.max_amount);
            if self.current_amount > 0 {
                self.exhausted = false;
            }
        }
    }

    pub fn fill_ratio(&self) -> f32 {
        if self.max_amount == 0 {
            return 0.0;
        }
        self.current_amount as f32 / self.max_amount as f32
    }
}