use std::collections::HashMap;

use bevy::prelude::*;

use crate::world::WorldPos;

#[derive(Resource, Debug, Clone, Default)]
pub struct CostMap {
    costs: HashMap<(i32, i32), f32>,
    default_cost: f32,
}

impl CostMap {
    pub fn new() -> Self {
        Self {
            costs: HashMap::new(),
            default_cost: 1.0,
        }
    }

    pub fn get_cost(&self, pos: WorldPos) -> f32 {
        self.costs
            .get(&(pos.x, pos.y))
            .copied()
            .unwrap_or(self.default_cost)
    }

    pub fn set_cost(&mut self, pos: WorldPos, cost: f32) {
        self.costs.insert((pos.x, pos.y), cost);
    }

    pub fn remove_cost(&mut self, pos: WorldPos) {
        self.costs.remove(&(pos.x, pos.y));
    }

    pub fn set_default(&mut self, cost: f32) {
        self.default_cost = cost;
    }
}
