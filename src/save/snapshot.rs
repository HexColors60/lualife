use serde::{Deserialize, Serialize};

use crate::core::TickNumber;
use crate::factions::FactionId;
use crate::resources::ResourceType;
use crate::world::WorldMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSnapshot {
    pub version: u32,
    pub tick: u64,
    pub factions: Vec<FactionSnapshot>,
    pub world: WorldSnapshot,
}

impl GameSnapshot {
    pub fn new(tick: &TickNumber) -> Self {
        Self {
            version: 1,
            tick: tick.0,
            factions: Vec::new(),
            world: WorldSnapshot::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSnapshot {
    pub id: FactionId,
    pub name: String,
    pub resources: std::collections::HashMap<ResourceType, u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub rooms: Vec<RoomSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSnapshot {
    pub x: u32,
    pub y: u32,
    pub terrain: Vec<Vec<String>>,
    pub mines: Vec<MineSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineSnapshot {
    pub id: u32,
    pub resource_type: ResourceType,
    pub amount: u32,
    pub max_amount: u32,
}
