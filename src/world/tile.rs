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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_default() {
        let tile = Tile::default();
        assert_eq!(tile.terrain, TerrainType::Plains);
        assert!(tile.walkable);
        assert!(tile.buildable);
        assert_eq!(tile.movement_cost, 1.0);
    }

    #[test]
    fn test_tile_new_plains() {
        let tile = Tile::new(TerrainType::Plains);
        assert_eq!(tile.terrain, TerrainType::Plains);
        assert!(tile.is_walkable());
        assert!(tile.is_buildable());
        assert_eq!(tile.movement_cost, 1.0);
    }

    #[test]
    fn test_tile_new_forest() {
        let tile = Tile::new(TerrainType::Forest);
        assert_eq!(tile.terrain, TerrainType::Forest);
        assert!(tile.is_walkable());
        assert!(!tile.is_buildable());
        assert_eq!(tile.movement_cost, 2.0);
    }

    #[test]
    fn test_tile_new_mountain() {
        let tile = Tile::new(TerrainType::Mountain);
        assert_eq!(tile.terrain, TerrainType::Mountain);
        assert!(!tile.is_walkable());
        assert!(!tile.is_buildable());
        assert!(tile.movement_cost.is_infinite());
    }

    #[test]
    fn test_tile_new_water() {
        let tile = Tile::new(TerrainType::Water);
        assert_eq!(tile.terrain, TerrainType::Water);
        assert!(!tile.is_walkable());
        assert!(!tile.is_buildable());
        assert!(tile.movement_cost.is_infinite());
    }

    #[test]
    fn test_tile_new_swamp() {
        let tile = Tile::new(TerrainType::Swamp);
        assert_eq!(tile.terrain, TerrainType::Swamp);
        assert!(tile.is_walkable());
        assert!(!tile.is_buildable());
        assert_eq!(tile.movement_cost, 3.0);
    }
}
