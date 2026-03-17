use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use super::{RoomCoord, Tile, TerrainType};
use super::ownership::RoomOwner;
use super::visibility::VisibilityState;
use crate::consts::ROOM_TILE_SIZE;

/// Room data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub coord: RoomCoord,
    pub tiles: Vec<Vec<Tile>>,
    pub mine_ids: IndexSet<u32>,
    pub building_ids: IndexSet<u32>,
    pub creep_ids: IndexSet<u32>,
    pub dropped_resource_ids: IndexSet<u32>,
    pub owner: RoomOwner,
    pub visibility: VisibilityState,
}

impl Room {
    pub fn new(coord: RoomCoord) -> Self {
        let tiles = vec![vec![Tile::default(); ROOM_TILE_SIZE as usize]; ROOM_TILE_SIZE as usize];
        Self {
            coord,
            tiles,
            mine_ids: IndexSet::new(),
            building_ids: IndexSet::new(),
            creep_ids: IndexSet::new(),
            dropped_resource_ids: IndexSet::new(),
            owner: RoomOwner::default(),
            visibility: VisibilityState::default(),
        }
    }

    pub fn get_tile(&self, local_x: usize, local_y: usize) -> Option<&Tile> {
        self.tiles.get(local_y).and_then(|row| row.get(local_x))
    }

    pub fn get_tile_mut(&mut self, local_x: usize, local_y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(local_y).and_then(|row| row.get_mut(local_x))
    }

    pub fn set_terrain(&mut self, local_x: usize, local_y: usize, terrain: TerrainType) {
        if let Some(tile) = self.get_tile_mut(local_x, local_y) {
            *tile = Tile::new(terrain);
        }
    }

    pub fn add_mine(&mut self, mine_id: u32) {
        self.mine_ids.insert(mine_id);
    }

    pub fn remove_mine(&mut self, mine_id: u32) {
        self.mine_ids.shift_remove(&mine_id);
    }

    pub fn add_building(&mut self, building_id: u32) {
        self.building_ids.insert(building_id);
    }

    pub fn remove_building(&mut self, building_id: u32) {
        self.building_ids.shift_remove(&building_id);
    }

    pub fn add_creep(&mut self, creep_id: u32) {
        self.creep_ids.insert(creep_id);
    }

    pub fn remove_creep(&mut self, creep_id: u32) {
        self.creep_ids.shift_remove(&creep_id);
    }

    pub fn walkable_tiles(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for y in 0..ROOM_TILE_SIZE as usize {
            for x in 0..ROOM_TILE_SIZE as usize {
                if let Some(tile) = self.get_tile(x, y) {
                    if tile.is_walkable() {
                        result.push((x, y));
                    }
                }
            }
        }
        result
    }
}