use crate::config::WorldgenConfig;
use crate::core::GameRng;
use crate::world::{Room, RoomCoord, TerrainType, Tile};
use crate::consts::{ROOM_GRID_X, ROOM_GRID_Y, ROOM_TILE_SIZE};

pub struct RoomLayoutGenerator;

impl RoomLayoutGenerator {
    pub fn generate_room_layout(
        coord: RoomCoord,
        config: &WorldgenConfig,
        rng: &mut GameRng,
    ) -> Vec<Vec<Tile>> {
        let size = ROOM_TILE_SIZE as usize;
        let mut tiles = Vec::with_capacity(size);

        // Generate base terrain
        for y in 0..size {
            let mut row = Vec::with_capacity(size);
            for x in 0..size {
                let terrain = Self::choose_terrain(coord.x, coord.y, x, y, config, rng);
                row.push(Tile::new(terrain));
            }
            tiles.push(row);
        }

        // Add some room-specific features
        Self::add_features(&mut tiles, coord, rng);

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
        let roll = rng.gen_range(0.0..1.0);
        let room_factor = ((room_x + room_y) % 5) as f32 / 5.0;
        let adjusted_roll = (roll + room_factor * 0.2) % 1.0;

        if adjusted_roll < config.plains_ratio {
            TerrainType::Plains
        } else if adjusted_roll < config.plains_ratio + config.forest_ratio {
            TerrainType::Forest
        } else if adjusted_roll < config.plains_ratio + config.forest_ratio + config.mountain_ratio {
            TerrainType::Mountain
        } else {
            TerrainType::Plains
        }
    }

    fn add_features(tiles: &mut Vec<Vec<Tile>>, _coord: RoomCoord, _rng: &mut GameRng) {
        // Add a small clearing in the center for potential building
        let center = ROOM_TILE_SIZE as usize / 2;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let x = (center as i32 + dx) as usize;
                let y = (center as i32 + dy) as usize;
                if let Some(row) = tiles.get_mut(y) {
                    if let Some(tile) = row.get_mut(x) {
                        *tile = Tile::new(TerrainType::Plains);
                    }
                }
            }
        }
    }
}