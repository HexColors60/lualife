use crate::config::WorldgenConfig;
use crate::consts::ROOM_TILE_SIZE;
use crate::core::GameRng;
use crate::world::{TerrainType, Tile};

pub struct TerrainGenerator;

impl TerrainGenerator {
    pub fn generate_room_terrain(
        room_x: u32,
        room_y: u32,
        config: &WorldgenConfig,
        rng: &mut GameRng,
    ) -> Vec<Vec<Tile>> {
        let size = ROOM_TILE_SIZE as usize;
        let mut tiles = Vec::with_capacity(size);

        for y in 0..size {
            let mut row = Vec::with_capacity(size);
            for x in 0..size {
                let terrain = Self::choose_terrain(room_x, room_y, x, y, config, rng);
                row.push(Tile::new(terrain));
            }
            tiles.push(row);
        }

        tiles
    }

    fn choose_terrain(
        room_x: u32,
        room_y: u32,
        _local_x: usize,
        _local_y: usize,
        config: &WorldgenConfig,
        rng: &mut GameRng,
    ) -> TerrainType {
        // Simple terrain generation based on position and randomness
        let roll = rng.gen_range(0.0..1.0);

        // Add some variation based on room position
        let room_factor = ((room_x + room_y) % 5) as f32 / 5.0;
        let adjusted_roll = (roll + room_factor * 0.2) % 1.0;

        if adjusted_roll < config.plains_ratio {
            TerrainType::Plains
        } else if adjusted_roll < config.plains_ratio + config.forest_ratio {
            TerrainType::Forest
        } else if adjusted_roll < config.plains_ratio + config.forest_ratio + config.mountain_ratio
        {
            TerrainType::Mountain
        } else {
            TerrainType::Plains
        }
    }
}
