use serde::{Deserialize, Serialize};

use crate::factions::FactionId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisibilityState {
    /// Factions that can see this room
    pub visible_to: Vec<FactionId>,
    /// Whether this room has been discovered by any faction
    pub discovered: bool,
}

impl VisibilityState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_visible(&mut self, faction_id: FactionId) {
        if !self.visible_to.contains(&faction_id) {
            self.visible_to.push(faction_id);
        }
        self.discovered = true;
    }

    pub fn set_invisible(&mut self, faction_id: FactionId) {
        self.visible_to.retain(|&id| id != faction_id);
    }

    pub fn is_visible_to(&self, faction_id: FactionId) -> bool {
        self.visible_to.contains(&faction_id)
    }

    pub fn clear_visibility(&mut self) {
        self.visible_to.clear();
    }
}