use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Road;

impl Road {
    pub fn movement_cost_reduction() -> f32 {
        0.5 // Roads reduce movement cost by 50%
    }
}