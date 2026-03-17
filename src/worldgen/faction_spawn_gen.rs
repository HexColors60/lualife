use crate::config::WorldgenConfig;
use crate::consts::{DEFAULT_AI_COUNT, ROOM_GRID_X, ROOM_GRID_Y};
use crate::core::GameRng;
use crate::factions::FactionId;
use crate::world::RoomCoord;

#[derive(Debug, Clone)]
pub struct FactionSpawn {
    pub faction_id: FactionId,
    pub room_coord: RoomCoord,
}

pub struct FactionSpawnGenerator;

impl FactionSpawnGenerator {
    pub fn generate_spawns(_config: &WorldgenConfig, rng: &mut GameRng) -> Vec<FactionSpawn> {
        let mut spawns = Vec::new();
        let faction_count = DEFAULT_AI_COUNT;

        // Calculate grid positions for fair distribution
        let grid_size = (faction_count as f64).sqrt().ceil() as u32;
        let spacing_x = ROOM_GRID_X / grid_size;
        let spacing_y = ROOM_GRID_Y / grid_size;

        let mut faction_id: u16 = 0;
        for gy in 0..grid_size {
            for gx in 0..grid_size {
                if faction_id as usize >= faction_count {
                    break;
                }

                let base_x = gx * spacing_x + spacing_x / 2;
                let base_y = gy * spacing_y + spacing_y / 2;

                // Add some randomness
                let offset_x = rng.gen_range(0..(spacing_x / 2).max(1));
                let offset_y = rng.gen_range(0..(spacing_y / 2).max(1));

                let room_x = (base_x + offset_x).min(ROOM_GRID_X - 1);
                let room_y = (base_y + offset_y).min(ROOM_GRID_Y - 1);

                spawns.push(FactionSpawn {
                    faction_id: FactionId(faction_id),
                    room_coord: RoomCoord::new(room_x, room_y),
                });

                faction_id += 1;
            }
        }

        spawns
    }
}
