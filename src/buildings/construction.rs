use bevy::prelude::*;

use super::BuildingType;
use crate::factions::FactionId;

#[derive(Debug, Clone, Component)]
pub struct ConstructionSite {
    pub building_type: BuildingType,
    pub faction_id: FactionId,
    pub progress: f32,
    pub required_progress: f32,
}

impl ConstructionSite {
    pub fn new(building_type: BuildingType, faction_id: FactionId) -> Self {
        Self {
            building_type,
            faction_id,
            progress: 0.0,
            required_progress: 100.0,
        }
    }

    pub fn add_progress(&mut self, amount: f32) {
        self.progress = (self.progress + amount).min(self.required_progress);
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= self.required_progress
    }

    pub fn progress_ratio(&self) -> f32 {
        self.progress / self.required_progress
    }
}
