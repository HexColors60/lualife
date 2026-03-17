use bevy::prelude::*;

use crate::factions::FactionId;

#[derive(Resource, Debug, Clone, Default)]
pub struct Scheduler {
    /// Current faction index for staggered execution
    pub current_faction_index: usize,
    /// Total factions to schedule
    pub faction_count: usize,
    /// Tick budget per faction
    pub budget_per_faction: usize,
}

impl Scheduler {
    pub fn new(faction_count: usize, budget_per_faction: usize) -> Self {
        Self {
            current_faction_index: 0,
            faction_count,
            budget_per_faction,
        }
    }

    /// Get the next faction to execute this tick
    pub fn next_faction(&mut self) -> Option<FactionId> {
        if self.faction_count == 0 {
            return None;
        }

        let faction_id = FactionId(self.current_faction_index as u16);
        self.current_faction_index = (self.current_faction_index + 1) % self.faction_count;
        Some(faction_id)
    }

    /// Reset for new tick cycle
    pub fn reset(&mut self) {
        self.current_faction_index = 0;
    }

    /// Check if all factions have been scheduled this tick
    pub fn is_cycle_complete(&self) -> bool {
        self.current_faction_index == 0
    }
}
