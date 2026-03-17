use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TerrainType {
    #[default]
    Plains,
    Forest,
    Mountain,
    Water,
    Swamp,
}

impl TerrainType {
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            TerrainType::Plains => (144, 238, 144), // Light green
            TerrainType::Forest => (34, 139, 34),    // Forest green
            TerrainType::Mountain => (139, 137, 137), // Gray
            TerrainType::Water => (65, 105, 225),    // Royal blue
            TerrainType::Swamp => (85, 107, 47),     // Dark olive green
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TerrainType::Plains => "plains",
            TerrainType::Forest => "forest",
            TerrainType::Mountain => "mountain",
            TerrainType::Water => "water",
            TerrainType::Swamp => "swamp",
        }
    }
}