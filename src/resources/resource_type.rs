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
