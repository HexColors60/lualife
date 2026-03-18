use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ResourceType {
    #[default]
    Power,
    Iron,
    Copper,
    Silicon,
    Crystal,
    Carbon,
    Stone,
    Sulfur,
    Water,
    Biomass,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::Power => "power",
            ResourceType::Iron => "iron",
            ResourceType::Copper => "copper",
            ResourceType::Silicon => "silicon",
            ResourceType::Crystal => "crystal",
            ResourceType::Carbon => "carbon",
            ResourceType::Stone => "stone",
            ResourceType::Sulfur => "sulfur",
            ResourceType::Water => "water",
            ResourceType::Biomass => "biomass",
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            ResourceType::Power => (255, 255, 0),     // Yellow
            ResourceType::Iron => (128, 128, 128),    // Gray
            ResourceType::Copper => (184, 115, 51),   // Copper
            ResourceType::Silicon => (192, 192, 192), // Silver
            ResourceType::Crystal => (0, 255, 255),   // Cyan
            ResourceType::Carbon => (54, 69, 79),     // Charcoal
            ResourceType::Stone => (169, 169, 169),   // Dark gray
            ResourceType::Sulfur => (255, 215, 0),    // Gold
            ResourceType::Water => (0, 191, 255),     // Deep sky blue
            ResourceType::Biomass => (0, 128, 0),     // Green
        }
    }

    pub fn all() -> [ResourceType; 10] {
        [
            ResourceType::Power,
            ResourceType::Iron,
            ResourceType::Copper,
            ResourceType::Silicon,
            ResourceType::Crystal,
            ResourceType::Carbon,
            ResourceType::Stone,
            ResourceType::Sulfur,
            ResourceType::Water,
            ResourceType::Biomass,
        ]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "power" => Some(ResourceType::Power),
            "iron" => Some(ResourceType::Iron),
            "copper" => Some(ResourceType::Copper),
            "silicon" => Some(ResourceType::Silicon),
            "crystal" => Some(ResourceType::Crystal),
            "carbon" => Some(ResourceType::Carbon),
            "stone" => Some(ResourceType::Stone),
            "sulfur" => Some(ResourceType::Sulfur),
            "water" => Some(ResourceType::Water),
            "biomass" => Some(ResourceType::Biomass),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_default() {
        let resource = ResourceType::default();
        assert_eq!(resource, ResourceType::Power);
    }

    #[test]
    fn test_resource_type_name() {
        assert_eq!(ResourceType::Power.name(), "power");
        assert_eq!(ResourceType::Iron.name(), "iron");
        assert_eq!(ResourceType::Copper.name(), "copper");
        assert_eq!(ResourceType::Silicon.name(), "silicon");
        assert_eq!(ResourceType::Crystal.name(), "crystal");
        assert_eq!(ResourceType::Carbon.name(), "carbon");
        assert_eq!(ResourceType::Stone.name(), "stone");
        assert_eq!(ResourceType::Sulfur.name(), "sulfur");
        assert_eq!(ResourceType::Water.name(), "water");
        assert_eq!(ResourceType::Biomass.name(), "biomass");
    }

    #[test]
    fn test_resource_type_from_name() {
        assert_eq!(ResourceType::from_name("power"), Some(ResourceType::Power));
        assert_eq!(ResourceType::from_name("iron"), Some(ResourceType::Iron));
        assert_eq!(ResourceType::from_name("copper"), Some(ResourceType::Copper));
        assert_eq!(ResourceType::from_name("silicon"), Some(ResourceType::Silicon));
        assert_eq!(ResourceType::from_name("crystal"), Some(ResourceType::Crystal));
        assert_eq!(ResourceType::from_name("carbon"), Some(ResourceType::Carbon));
        assert_eq!(ResourceType::from_name("stone"), Some(ResourceType::Stone));
        assert_eq!(ResourceType::from_name("sulfur"), Some(ResourceType::Sulfur));
        assert_eq!(ResourceType::from_name("water"), Some(ResourceType::Water));
        assert_eq!(ResourceType::from_name("biomass"), Some(ResourceType::Biomass));
        assert_eq!(ResourceType::from_name("unknown"), None);
    }

    #[test]
    fn test_resource_type_roundtrip() {
        for resource in ResourceType::all() {
            let name = resource.name();
            let parsed = ResourceType::from_name(name);
            assert_eq!(parsed, Some(resource));
        }
    }

    #[test]
    fn test_resource_type_color() {
        assert_eq!(ResourceType::Power.color(), (255, 255, 0));
        assert_eq!(ResourceType::Iron.color(), (128, 128, 128));
        assert_eq!(ResourceType::Copper.color(), (184, 115, 51));
        assert_eq!(ResourceType::Silicon.color(), (192, 192, 192));
        assert_eq!(ResourceType::Crystal.color(), (0, 255, 255));
        assert_eq!(ResourceType::Carbon.color(), (54, 69, 79));
        assert_eq!(ResourceType::Stone.color(), (169, 169, 169));
        assert_eq!(ResourceType::Sulfur.color(), (255, 215, 0));
        assert_eq!(ResourceType::Water.color(), (0, 191, 255));
        assert_eq!(ResourceType::Biomass.color(), (0, 128, 0));
    }

    #[test]
    fn test_resource_type_all() {
        let all = ResourceType::all();
        assert_eq!(all.len(), 10);
        assert!(all.contains(&ResourceType::Power));
        assert!(all.contains(&ResourceType::Iron));
        assert!(all.contains(&ResourceType::Copper));
        assert!(all.contains(&ResourceType::Silicon));
        assert!(all.contains(&ResourceType::Crystal));
        assert!(all.contains(&ResourceType::Carbon));
        assert!(all.contains(&ResourceType::Stone));
        assert!(all.contains(&ResourceType::Sulfur));
        assert!(all.contains(&ResourceType::Water));
        assert!(all.contains(&ResourceType::Biomass));
    }

    #[test]
    fn test_resource_type_equality() {
        assert_eq!(ResourceType::Iron, ResourceType::Iron);
        assert_ne!(ResourceType::Iron, ResourceType::Copper);
    }

    #[test]
    fn test_resource_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ResourceType::Iron);
        set.insert(ResourceType::Copper);
        set.insert(ResourceType::Iron);
        assert_eq!(set.len(), 2);
    }
}
