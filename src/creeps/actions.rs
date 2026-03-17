use serde::{Deserialize, Serialize};

use crate::resources::ResourceType;
use crate::world::WorldPos;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreepAction {
    Idle,
    MoveTo {
        target: WorldPos,
    },
    Mine {
        mine_id: u32,
    },
    Build {
        building_id: u32,
    },
    Repair {
        building_id: u32,
    },
    Attack {
        target_id: u32,
    },
    Transfer {
        target_id: u32,
        resource: ResourceType,
        amount: u32,
    },
    Withdraw {
        target_id: u32,
        resource: ResourceType,
        amount: u32,
    },
    Pickup {
        resource_type: ResourceType,
    },
    Drop {
        resource: ResourceType,
        amount: u32,
    },
    Harvest {
        source_id: u32,
    },
    Upgrade {
        controller_id: u32,
    },
    Heal {
        target_id: u32,
    },
}

impl Default for CreepAction {
    fn default() -> Self {
        Self::Idle
    }
}

impl CreepAction {
    pub fn is_idle(&self) -> bool {
        matches!(self, CreepAction::Idle)
    }

    pub fn is_moving(&self) -> bool {
        matches!(self, CreepAction::MoveTo { .. })
    }
}
