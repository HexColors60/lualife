use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::terrain::TerrainType;

/// A single world tile
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Tile {
    pub terrain: TerrainType,
    pub walkable: bool,
    pub buildable: bool,
    pub movement_cost: f32,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            terrain: TerrainType::Plains,
            walkable: true,
            buildable: true,
            movement_cost: 1.0,
        }
    }
}

impl Tile {
    pub fn new(terrain: TerrainType) -> Self {
        let (walkable, buildable, movement_cost) = match terrain {
            TerrainType::Plains => (true, true, 1.0),
            TerrainType::Forest => (true, false, 2.0),
            TerrainType::Mountain => (false, false, f32::INFINITY),
            TerrainType::Water => (false, false, f32::INFINITY),
            TerrainType::Swamp => (true, false, 3.0),
        };
        Self {
            terrain,
            walkable,
            buildable,
            movement_cost,
        }
    }

    pub fn is_walkable(&self) -> bool {
        self.walkable
    }

    pub fn is_buildable(&self) -> bool {
        self.buildable
    }
}
