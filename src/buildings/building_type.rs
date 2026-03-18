use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum BuildingType {
    #[default]
    BaseCore,
    Spawn,
    Storage,
    PowerDepot,
    MineExtractor,
    Refinery,
    Workshop,
    Wall,
    Tower,
    Road,
    ScriptRelay,
    Scanner,
    RepairBay,
    Factory,
    Lab,
    Barracks,
}

impl BuildingType {
    pub fn name(&self) -> &'static str {
        match self {
            BuildingType::BaseCore => "base_core",
            BuildingType::Spawn => "spawn",
            BuildingType::Storage => "storage",
            BuildingType::PowerDepot => "power_depot",
            BuildingType::MineExtractor => "mine_extractor",
            BuildingType::Refinery => "refinery",
            BuildingType::Workshop => "workshop",
            BuildingType::Wall => "wall",
            BuildingType::Tower => "tower",
            BuildingType::Road => "road",
            BuildingType::ScriptRelay => "script_relay",
            BuildingType::Scanner => "scanner",
            BuildingType::RepairBay => "repair_bay",
            BuildingType::Factory => "factory",
            BuildingType::Lab => "lab",
            BuildingType::Barracks => "barracks",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "base_core" => Some(BuildingType::BaseCore),
            "spawn" => Some(BuildingType::Spawn),
            "storage" => Some(BuildingType::Storage),
            "power_depot" => Some(BuildingType::PowerDepot),
            "mine_extractor" => Some(BuildingType::MineExtractor),
            "refinery" => Some(BuildingType::Refinery),
            "workshop" => Some(BuildingType::Workshop),
            "wall" => Some(BuildingType::Wall),
            "tower" => Some(BuildingType::Tower),
            "road" => Some(BuildingType::Road),
            "script_relay" => Some(BuildingType::ScriptRelay),
            "scanner" => Some(BuildingType::Scanner),
            "repair_bay" => Some(BuildingType::RepairBay),
            "factory" => Some(BuildingType::Factory),
            "lab" => Some(BuildingType::Lab),
            "barracks" => Some(BuildingType::Barracks),
            _ => None,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            BuildingType::BaseCore => (255, 215, 0),        // Gold
            BuildingType::Spawn => (0, 255, 127),           // Spring green
            BuildingType::Storage => (139, 90, 43),         // Brown
            BuildingType::PowerDepot => (255, 255, 0),      // Yellow
            BuildingType::MineExtractor => (128, 128, 128), // Gray
            BuildingType::Refinery => (255, 140, 0),        // Dark orange
            BuildingType::Workshop => (106, 90, 205),       // Slate blue
            BuildingType::Wall => (105, 105, 105),          // Dim gray
            BuildingType::Tower => (178, 34, 34),           // Firebrick
            BuildingType::Road => (160, 82, 45),            // Sienna
            BuildingType::ScriptRelay => (75, 0, 130),      // Indigo
            BuildingType::Scanner => (0, 206, 209),         // Dark turquoise
            BuildingType::RepairBay => (50, 205, 50),       // Lime green
            BuildingType::Factory => (70, 130, 180),        // Steel blue
            BuildingType::Lab => (148, 0, 211),             // Dark violet
            BuildingType::Barracks => (220, 20, 60),        // Crimson
        }
    }

    pub fn base_cost(&self) -> Vec<(crate::resources::ResourceType, u32)> {
        use crate::resources::ResourceType;

        match self {
            BuildingType::BaseCore => vec![(ResourceType::Iron, 500), (ResourceType::Stone, 300)],
            BuildingType::Spawn => vec![(ResourceType::Iron, 300), (ResourceType::Power, 200)],
            BuildingType::Storage => vec![(ResourceType::Iron, 200), (ResourceType::Stone, 100)],
            BuildingType::PowerDepot => vec![(ResourceType::Iron, 100), (ResourceType::Copper, 50)],
            BuildingType::MineExtractor => {
                vec![(ResourceType::Iron, 150), (ResourceType::Copper, 50)]
            }
            BuildingType::Refinery => vec![(ResourceType::Iron, 200), (ResourceType::Copper, 100)],
            BuildingType::Workshop => vec![(ResourceType::Iron, 150), (ResourceType::Silicon, 50)],
            BuildingType::Wall => vec![(ResourceType::Stone, 50)],
            BuildingType::Tower => vec![(ResourceType::Iron, 200), (ResourceType::Crystal, 50)],
            BuildingType::Road => vec![(ResourceType::Stone, 10)],
            BuildingType::ScriptRelay => {
                vec![(ResourceType::Iron, 100), (ResourceType::Silicon, 100)]
            }
            BuildingType::Scanner => vec![(ResourceType::Iron, 100), (ResourceType::Crystal, 50)],
            BuildingType::RepairBay => vec![(ResourceType::Iron, 150), (ResourceType::Copper, 50)],
            BuildingType::Factory => vec![(ResourceType::Iron, 300), (ResourceType::Silicon, 100)],
            BuildingType::Lab => vec![(ResourceType::Iron, 200), (ResourceType::Crystal, 100)],
            BuildingType::Barracks => vec![(ResourceType::Iron, 250), (ResourceType::Power, 100)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_type_default() {
        let building = BuildingType::default();
        assert_eq!(building, BuildingType::BaseCore);
    }

    #[test]
    fn test_building_type_name() {
        assert_eq!(BuildingType::BaseCore.name(), "base_core");
        assert_eq!(BuildingType::Spawn.name(), "spawn");
        assert_eq!(BuildingType::Storage.name(), "storage");
        assert_eq!(BuildingType::PowerDepot.name(), "power_depot");
        assert_eq!(BuildingType::MineExtractor.name(), "mine_extractor");
        assert_eq!(BuildingType::Refinery.name(), "refinery");
        assert_eq!(BuildingType::Workshop.name(), "workshop");
        assert_eq!(BuildingType::Wall.name(), "wall");
        assert_eq!(BuildingType::Tower.name(), "tower");
        assert_eq!(BuildingType::Road.name(), "road");
        assert_eq!(BuildingType::ScriptRelay.name(), "script_relay");
        assert_eq!(BuildingType::Scanner.name(), "scanner");
        assert_eq!(BuildingType::RepairBay.name(), "repair_bay");
        assert_eq!(BuildingType::Factory.name(), "factory");
        assert_eq!(BuildingType::Lab.name(), "lab");
        assert_eq!(BuildingType::Barracks.name(), "barracks");
    }

    #[test]
    fn test_building_type_from_name() {
        assert_eq!(BuildingType::from_name("base_core"), Some(BuildingType::BaseCore));
        assert_eq!(BuildingType::from_name("spawn"), Some(BuildingType::Spawn));
        assert_eq!(BuildingType::from_name("storage"), Some(BuildingType::Storage));
        assert_eq!(BuildingType::from_name("unknown"), None);
    }

    #[test]
    fn test_building_type_roundtrip() {
        for building in [
            BuildingType::BaseCore,
            BuildingType::Spawn,
            BuildingType::Storage,
            BuildingType::PowerDepot,
            BuildingType::MineExtractor,
            BuildingType::Refinery,
            BuildingType::Workshop,
            BuildingType::Wall,
            BuildingType::Tower,
            BuildingType::Road,
            BuildingType::ScriptRelay,
            BuildingType::Scanner,
            BuildingType::RepairBay,
            BuildingType::Factory,
            BuildingType::Lab,
            BuildingType::Barracks,
        ] {
            let name = building.name();
            let parsed = BuildingType::from_name(name);
            assert_eq!(parsed, Some(building));
        }
    }

    #[test]
    fn test_building_type_color() {
        assert_eq!(BuildingType::BaseCore.color(), (255, 215, 0));
        assert_eq!(BuildingType::Spawn.color(), (0, 255, 127));
        assert_eq!(BuildingType::Storage.color(), (139, 90, 43));
    }

    #[test]
    fn test_building_type_base_cost() {
        use crate::resources::ResourceType;

        let cost = BuildingType::Wall.base_cost();
        assert_eq!(cost.len(), 1);
        assert_eq!(cost[0], (ResourceType::Stone, 50));

        let cost = BuildingType::BaseCore.base_cost();
        assert_eq!(cost.len(), 2);
        assert_eq!(cost[0], (ResourceType::Iron, 500));
        assert_eq!(cost[1], (ResourceType::Stone, 300));
    }

    #[test]
    fn test_building_type_equality() {
        assert_eq!(BuildingType::Spawn, BuildingType::Spawn);
        assert_ne!(BuildingType::Spawn, BuildingType::Storage);
    }

    #[test]
    fn test_building_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BuildingType::Spawn);
        set.insert(BuildingType::Storage);
        set.insert(BuildingType::Spawn);
        assert_eq!(set.len(), 2);
    }
}
