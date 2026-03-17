use indexmap::IndexSet;

use super::{
    FactionSpawn, FactionSpawnGenerator, MineDistributor, MinePlacement, RoomLayoutGenerator,
};
use crate::config::WorldgenConfig;
use crate::consts::{ROOM_GRID_X, ROOM_GRID_Y};
use crate::core::GameRng;
use crate::world::{Room, RoomCoord, Tile};

#[derive(Debug, Clone)]
pub struct GeneratedRoom {
    pub coord: RoomCoord,
    pub tiles: Vec<Vec<Tile>>,
    pub mine_ids: IndexSet<u32>,
}

#[derive(Debug, Clone)]
pub struct GeneratedWorld {
    pub rooms: Vec<GeneratedRoom>,
    pub faction_spawns: Vec<FactionSpawn>,
    pub mine_placements: Vec<(RoomCoord, Vec<MinePlacement>)>,
}

pub struct WorldGenerator;

impl WorldGenerator {
    pub fn generate(config: &WorldgenConfig, rng: &mut GameRng) -> GeneratedWorld {
        let mut rooms = Vec::new();
        let mut mine_placements = Vec::new();
        let mut next_mine_id: u32 = 1;

        // Generate all rooms
        for y in 0..ROOM_GRID_Y {
            for x in 0..ROOM_GRID_X {
                let coord = RoomCoord::new(x, y);
                let tiles = RoomLayoutGenerator::generate_room_layout(coord, config, rng);

                // Generate mines for this room
                let mines = MineDistributor::assign_room_mines(x, y, config, rng);
                let mut mine_ids = IndexSet::new();

                for _mine in &mines {
                    mine_ids.insert(next_mine_id);
                    next_mine_id += 1;
                }

                if !mines.is_empty() {
                    mine_placements.push((coord, mines));
                }

                rooms.push(GeneratedRoom {
                    coord,
                    tiles,
                    mine_ids,
                });
            }
        }

        // Generate faction spawn positions
        let faction_spawns = FactionSpawnGenerator::generate_spawns(config, rng);

        GeneratedWorld {
            rooms,
            faction_spawns,
            mine_placements,
        }
    }
}
