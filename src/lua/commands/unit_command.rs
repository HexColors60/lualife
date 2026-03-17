use serde::{Deserialize, Serialize};

use crate::resources::ResourceType;
use crate::world::WorldPos;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitCommand {
    MoveTo { creep_id: u32, target: WorldPos },
    Mine { creep_id: u32, mine_id: u32 },
    Build { creep_id: u32, building_type: String, x: i32, y: i32 },
    Attack { creep_id: u32, target_id: u32 },
    Transfer { creep_id: u32, target_id: u32, resource: ResourceType, amount: u32 },
    Withdraw { creep_id: u32, target_id: u32, resource: ResourceType, amount: u32 },
    Pickup { creep_id: u32, resource_type: ResourceType },
    Drop { creep_id: u32, resource: ResourceType, amount: u32 },
}