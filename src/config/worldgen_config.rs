use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::consts::*;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct WorldgenConfig {
    /// Random seed for world generation (0 = random)
    pub seed: u64,

    /// Minimum mines per room
    pub mines_per_room_min: usize,

    /// Maximum mines per room
    pub mines_per_room_max: usize,

    /// Power mine rarity (higher = rarer)
    pub power_mine_rarity: f32,

    /// Terrain plains ratio
    pub plains_ratio: f32,

    /// Terrain forest ratio
    pub forest_ratio: f32,

    /// Terrain mountain ratio
    pub mountain_ratio: f32,

    /// Minimum distance between faction spawns
    pub faction_spawn_min_distance: u32,
}

impl Default for WorldgenConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            mines_per_room_min: MINES_PER_ROOM_MIN,
            mines_per_room_max: MINES_PER_ROOM_MAX,
            power_mine_rarity: 0.3,
            plains_ratio: 0.6,
            forest_ratio: 0.25,
            mountain_ratio: 0.15,
            faction_spawn_min_distance: 3,
        }
    }
}
