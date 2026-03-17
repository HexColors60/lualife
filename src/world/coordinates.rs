use serde::{Deserialize, Serialize};

use crate::consts::*;
use crate::error::GameError;

/// World position in tile coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct WorldPos {
    pub x: i32,
    pub y: i32,
}

impl WorldPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_room_pos(&self) -> RoomPos {
        RoomPos::new(
            (self.x.div_euclid(ROOM_TILE_SIZE as i32)) as u32,
            (self.y.div_euclid(ROOM_TILE_SIZE as i32)) as u32,
        )
    }

    pub fn to_local(&self) -> (u32, u32) {
        let size = ROOM_TILE_SIZE as i32;
        (
            self.x.rem_euclid(size) as u32,
            self.y.rem_euclid(size) as u32,
        )
    }

    pub fn is_valid(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < WORLD_TILES_X as i32 && self.y < WORLD_TILES_Y as i32
    }

    pub fn clamp(&self) -> Self {
        Self::new(
            self.x.clamp(0, WORLD_TILES_X as i32 - 1),
            self.y.clamp(0, WORLD_TILES_Y as i32 - 1),
        )
    }

    pub fn distance(&self, other: &WorldPos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(&self, other: &WorldPos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Room coordinate in the room grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct RoomCoord {
    pub x: u32,
    pub y: u32,
}

impl RoomCoord {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_index(index: usize) -> Self {
        let x = (index % ROOM_GRID_X as usize) as u32;
        let y = (index / ROOM_GRID_X as usize) as u32;
        Self { x, y }
    }

    pub fn to_index(&self) -> usize {
        (self.y as usize) * (ROOM_GRID_X as usize) + (self.x as usize)
    }

    pub fn is_valid(&self) -> bool {
        self.x < ROOM_GRID_X && self.y < ROOM_GRID_Y
    }

    pub fn to_world_origin(&self) -> WorldPos {
        WorldPos::new(
            (self.x * ROOM_TILE_SIZE) as i32,
            (self.y * ROOM_TILE_SIZE) as i32,
        )
    }

    pub fn neighbors(&self) -> Vec<RoomCoord> {
        let mut neighbors = Vec::new();
        for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let nx = self.x as i32 + dx;
            let ny = self.y as i32 + dy;
            if nx >= 0 && ny >= 0 {
                let coord = RoomCoord::new(nx as u32, ny as u32);
                if coord.is_valid() {
                    neighbors.push(coord);
                }
            }
        }
        neighbors
    }
}

/// Position within a room (local tile coordinates)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct RoomPos {
    pub room_x: u32,
    pub room_y: u32,
}

impl RoomPos {
    pub fn new(room_x: u32, room_y: u32) -> Self {
        Self { room_x, room_y }
    }

    pub fn to_world(&self, local_x: u32, local_y: u32) -> WorldPos {
        WorldPos::new(
            (self.room_x * ROOM_TILE_SIZE + local_x) as i32,
            (self.room_y * ROOM_TILE_SIZE + local_y) as i32,
        )
    }

    pub fn from_world(pos: &WorldPos) -> Self {
        Self::new(
            (pos.x / ROOM_TILE_SIZE as i32) as u32,
            (pos.y / ROOM_TILE_SIZE as i32) as u32,
        )
    }
}

impl From<WorldPos> for RoomCoord {
    fn from(pos: WorldPos) -> Self {
        RoomCoord::new(
            (pos.x / ROOM_TILE_SIZE as i32) as u32,
            (pos.y / ROOM_TILE_SIZE as i32) as u32,
        )
    }
}

impl From<RoomCoord> for RoomPos {
    fn from(coord: RoomCoord) -> Self {
        RoomPos::new(coord.x, coord.y)
    }
}
