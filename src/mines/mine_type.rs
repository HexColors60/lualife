use serde::{Deserialize, Serialize};

use crate::resources::ResourceType;

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct MineType {
    pub resource_type: ResourceType,
    pub base_extraction_rate: u32,
    pub regen_rate: f32,
    pub rarity: f32,
}

impl MineType {
    pub fn new(resource_type: ResourceType) -> Self {
        let (base_extraction_rate, regen_rate, rarity) = match resource_type {
            ResourceType::Power => (15, 0.05, 0.3),
            ResourceType::Iron => (20, 0.1, 0.5),
            ResourceType::Copper => (18, 0.08, 0.5),
            ResourceType::Silicon => (12, 0.06, 0.4),
            ResourceType::Crystal => (8, 0.04, 0.2),
            ResourceType::Carbon => (20, 0.12, 0.6),
            ResourceType::Stone => (25, 0.15, 0.7),
            ResourceType::Sulfur => (10, 0.05, 0.25),
            ResourceType::Water => (30, 0.2, 0.6),
            ResourceType::Biomass => (15, 0.1, 0.4),
        };

        Self {
            resource_type,
            base_extraction_rate,
            regen_rate,
            rarity,
        }
    }

    pub fn for_resource(resource_type: ResourceType) -> Self {
        Self::new(resource_type)
    }
}
