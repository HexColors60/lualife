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
            TerrainType::Plains => (144, 238, 144),
            TerrainType::Forest => (34, 139, 34),
            TerrainType::Mountain => (139, 137, 137),
            TerrainType::Water => (65, 105, 225),
            TerrainType::Swamp => (85, 107, 47),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_default() {
        let terrain = TerrainType::default();
        assert_eq!(terrain, TerrainType::Plains);
    }

    #[test]
    fn test_terrain_name() {
        assert_eq!(TerrainType::Plains.name(), "plains");
        assert_eq!(TerrainType::Forest.name(), "forest");
        assert_eq!(TerrainType::Mountain.name(), "mountain");
        assert_eq!(TerrainType::Water.name(), "water");
        assert_eq!(TerrainType::Swamp.name(), "swamp");
    }

    #[test]
    fn test_terrain_color() {
        assert_eq!(TerrainType::Plains.color(), (144, 238, 144));
        assert_eq!(TerrainType::Forest.color(), (34, 139, 34));
        assert_eq!(TerrainType::Mountain.color(), (139, 137, 137));
        assert_eq!(TerrainType::Water.color(), (65, 105, 225));
        assert_eq!(TerrainType::Swamp.color(), (85, 107, 47));
    }

    #[test]
    fn test_terrain_equality() {
        assert_eq!(TerrainType::Plains, TerrainType::Plains);
        assert_ne!(TerrainType::Plains, TerrainType::Forest);
    }

    #[test]
    fn test_terrain_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TerrainType::Plains);
        set.insert(TerrainType::Forest);
        set.insert(TerrainType::Plains);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_terrain_clone() {
        let terrain = TerrainType::Mountain;
        let cloned = terrain.clone();
        assert_eq!(terrain, cloned);
    }
}
