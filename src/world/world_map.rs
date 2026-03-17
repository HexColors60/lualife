use std::collections::HashMap;

use bevy::prelude::*;

use super::{Room, RoomCoord, Tile, WorldPos};
use crate::consts::*;
use crate::error::{GameError, GameResult};

/// The main world map resource
#[derive(Resource, Debug, Clone)]
pub struct WorldMap {
    rooms: HashMap<RoomCoord, Room>,
    total_rooms: usize,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldMap {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            total_rooms: 0,
        }
    }

    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    pub fn total_tiles(&self) -> usize {
        self.rooms.len() * (ROOM_TILE_SIZE as usize) * (ROOM_TILE_SIZE as usize)
    }

    pub fn add_room(&mut self, room: Room) {
        self.total_rooms += 1;
        self.rooms.insert(room.coord, room);
    }

    pub fn get_room(&self, coord: RoomCoord) -> Option<&Room> {
        self.rooms.get(&coord)
    }

    pub fn get_room_mut(&mut self, coord: RoomCoord) -> Option<&mut Room> {
        self.rooms.get_mut(&coord)
    }

    pub fn get_room_at(&self, pos: WorldPos) -> Option<&Room> {
        let coord = RoomCoord::from(pos);
        self.get_room(coord)
    }

    pub fn get_room_mut_at(&mut self, pos: WorldPos) -> Option<&mut Room> {
        let coord = RoomCoord::from(pos);
        self.get_room_mut(coord)
    }

    pub fn get_tile(&self, pos: WorldPos) -> Option<&Tile> {
        let coord = RoomCoord::from(pos);
        let (local_x, local_y) = pos.to_local();
        self.get_room(coord)
            .and_then(|room| room.get_tile(local_x as usize, local_y as usize))
    }

    pub fn get_tile_mut(&mut self, pos: WorldPos) -> Option<&mut Tile> {
        let coord = RoomCoord::from(pos);
        let (local_x, local_y) = pos.to_local();
        self.get_room_mut(coord)
            .and_then(|room| room.get_tile_mut(local_x as usize, local_y as usize))
    }

    pub fn is_walkable(&self, pos: WorldPos) -> bool {
        self.get_tile(pos).map(|t| t.is_walkable()).unwrap_or(false)
    }

    pub fn is_buildable(&self, pos: WorldPos) -> bool {
        self.get_tile(pos).map(|t| t.is_buildable()).unwrap_or(false)
    }

    pub fn all_rooms(&self) -> impl Iterator<Item = &Room> {
        self.rooms.values()
    }

    pub fn all_rooms_mut(&mut self) -> impl Iterator<Item = &mut Room> {
        self.rooms.values_mut()
    }

    pub fn room_coords(&self) -> impl Iterator<Item = RoomCoord> + '_ {
        self.rooms.keys().copied()
    }

    pub fn initialize_all_rooms(&mut self) {
        for y in 0..ROOM_GRID_Y {
            for x in 0..ROOM_GRID_X {
                let coord = RoomCoord::new(x, y);
                if !self.rooms.contains_key(&coord) {
                    self.add_room(Room::new(coord));
                }
            }
        }
    }
}