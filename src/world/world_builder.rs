use bevy::prelude::*;

use super::{Room, RoomCoord, WorldMap};
use crate::worldgen::GeneratedWorld;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn build_from_generated(generated: GeneratedWorld) -> WorldMap {
        let mut world_map = WorldMap::new();

        for room_data in generated.rooms {
            let room = Room {
                coord: room_data.coord,
                tiles: room_data.tiles,
                mine_ids: room_data.mine_ids,
                building_ids: Default::default(),
                creep_ids: Default::default(),
                dropped_resource_ids: Default::default(),
                owner: Default::default(),
                visibility: Default::default(),
            };
            world_map.add_room(room);
        }

        world_map
    }
}
