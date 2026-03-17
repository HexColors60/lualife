use serde::{Deserialize, Serialize};

use crate::factions::FactionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RoomOwner {
    pub faction_id: Option<FactionId>,
    pub controller_level: u8,
    pub reserved_by: Option<FactionId>,
}

impl RoomOwner {
    pub fn unowned() -> Self {
        Self::default()
    }

    pub fn owned_by(faction_id: FactionId) -> Self {
        Self {
            faction_id: Some(faction_id),
            controller_level: 1,
            reserved_by: None,
        }
    }

    pub fn is_owned(&self) -> bool {
        self.faction_id.is_some()
    }

    pub fn is_owned_by(&self, faction_id: FactionId) -> bool {
        self.faction_id == Some(faction_id)
    }
}