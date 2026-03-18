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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_pos_new() {
        let pos = WorldPos::new(10, 20);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
    }

    #[test]
    fn test_world_pos_to_room_pos() {
        let pos = WorldPos::new(10, 20);
        let room_pos = pos.to_room_pos();
        assert_eq!(room_pos.room_x, 1);
        assert_eq!(room_pos.room_y, 2);
    }

    #[test]
    fn test_world_pos_to_local() {
        let pos = WorldPos::new(10, 20);
        let (local_x, local_y) = pos.to_local();
        assert_eq!(local_x, 2);
        assert_eq!(local_y, 4);
    }

    #[test]
    fn test_world_pos_is_valid() {
        assert!(WorldPos::new(0, 0).is_valid());
        assert!(WorldPos::new(255, 255).is_valid());
        assert!(!WorldPos::new(-1, 0).is_valid());
        assert!(!WorldPos::new(0, -1).is_valid());
        assert!(!WorldPos::new(256, 0).is_valid());
        assert!(!WorldPos::new(0, 256).is_valid());
    }

    #[test]
    fn test_world_pos_clamp() {
        assert_eq!(WorldPos::new(-1, -1).clamp(), WorldPos::new(0, 0));
        assert_eq!(WorldPos::new(300, 300).clamp(), WorldPos::new(255, 255));
        assert_eq!(WorldPos::new(100, 100).clamp(), WorldPos::new(100, 100));
    }

    #[test]
    fn test_world_pos_distance() {
        let pos1 = WorldPos::new(0, 0);
        let pos2 = WorldPos::new(3, 4);
        assert!((pos1.distance(&pos2) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_world_pos_manhattan_distance() {
        let pos1 = WorldPos::new(0, 0);
        let pos2 = WorldPos::new(3, 4);
        assert_eq!(pos1.manhattan_distance(&pos2), 7);
    }

    #[test]
    fn test_room_coord_new() {
        let coord = RoomCoord::new(5, 10);
        assert_eq!(coord.x, 5);
        assert_eq!(coord.y, 10);
    }

    #[test]
    fn test_room_coord_from_index() {
        let coord = RoomCoord::from_index(0);
        assert_eq!(coord.x, 0);
        assert_eq!(coord.y, 0);

        let coord = RoomCoord::from_index(33);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 1);
    }

    #[test]
    fn test_room_coord_to_index() {
        let coord = RoomCoord::new(1, 1);
        assert_eq!(coord.to_index(), 33);
    }

    #[test]
    fn test_room_coord_is_valid() {
        assert!(RoomCoord::new(0, 0).is_valid());
        assert!(RoomCoord::new(31, 31).is_valid());
        assert!(!RoomCoord::new(32, 0).is_valid());
        assert!(!RoomCoord::new(0, 32).is_valid());
    }

    #[test]
    fn test_room_coord_to_world_origin() {
        let coord = RoomCoord::new(2, 3);
        let origin = coord.to_world_origin();
        assert_eq!(origin.x, 16);
        assert_eq!(origin.y, 24);
    }

    #[test]
    fn test_room_coord_neighbors() {
        let coord = RoomCoord::new(1, 1);
        let neighbors = coord.neighbors();
        assert_eq!(neighbors.len(), 4);

        let corner = RoomCoord::new(0, 0);
        let neighbors = corner.neighbors();
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn test_room_pos_new() {
        let pos = RoomPos::new(3, 5);
        assert_eq!(pos.room_x, 3);
        assert_eq!(pos.room_y, 5);
    }

    #[test]
    fn test_room_pos_to_world() {
        let pos = RoomPos::new(2, 3);
        let world = pos.to_world(4, 5);
        assert_eq!(world.x, 20);
        assert_eq!(world.y, 29);
    }

    #[test]
    fn test_room_pos_from_world() {
        let world = WorldPos::new(10, 20);
        let pos = RoomPos::from_world(&world);
        assert_eq!(pos.room_x, 1);
        assert_eq!(pos.room_y, 2);
    }

    #[test]
    fn test_world_pos_to_room_coord_conversion() {
        let pos = WorldPos::new(16, 24);
        let coord: RoomCoord = pos.into();
        assert_eq!(coord.x, 2);
        assert_eq!(coord.y, 3);
    }

    #[test]
    fn test_room_coord_to_room_pos_conversion() {
        let coord = RoomCoord::new(5, 7);
        let pos: RoomPos = coord.into();
        assert_eq!(pos.room_x, 5);
        assert_eq!(pos.room_y, 7);
    }
}
