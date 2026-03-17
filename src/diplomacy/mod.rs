use std::collections::{HashMap, HashSet};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;

/// Diplomatic stance between factions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiplomaticStance {
    Neutral,    // Default - no special relationship
    Allied,     // Friendly - share vision, no combat
    Enemy,      // Hostile - can attack
    Truce,      // Temporary peace
}

impl Default for DiplomaticStance {
    fn default() -> Self {
        Self::Neutral
    }
}

/// Alliance between multiple factions
#[derive(Debug, Clone, Component)]
pub struct Alliance {
    pub id: u32,
    pub name: String,
    pub members: HashSet<FactionId>,
    pub founder: FactionId,
}

impl Alliance {
    pub fn new(id: u32, name: String, founder: FactionId) -> Self {
        let mut members = HashSet::new();
        members.insert(founder);

        Self {
            id,
            name,
            members,
            founder,
        }
    }

    pub fn add_member(&mut self, faction: FactionId) {
        self.members.insert(faction);
    }

    pub fn remove_member(&mut self, faction: FactionId) {
        self.members.remove(&faction);
    }

    pub fn is_member(&self, faction: FactionId) -> bool {
        self.members.contains(&faction)
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// Diplomacy state for the game
#[derive(Resource, Debug, Clone, Default)]
pub struct DiplomacyState {
    /// Bilateral relationships between factions
    relationships: HashMap<(FactionId, FactionId), DiplomaticStance>,
    /// Active alliances
    alliances: HashMap<u32, Alliance>,
    alliance_counter: u32,
}

impl DiplomacyState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get all alliances
    pub fn get_alliances(&self) -> &HashMap<u32, Alliance> {
        &self.alliances
    }

    /// Get alliance count
    pub fn alliance_count(&self) -> usize {
        self.alliances.len()
    }

    /// Set the diplomatic stance between two factions
    pub fn set_stance(&mut self, faction1: FactionId, faction2: FactionId, stance: DiplomaticStance) {
        // Store both directions for easy lookup
        self.relationships.insert((faction1, faction2), stance);
        self.relationships.insert((faction2, faction1), stance);
    }

    /// Get the diplomatic stance between two factions
    pub fn get_stance(&self, faction1: FactionId, faction2: FactionId) -> DiplomaticStance {
        if faction1 == faction2 {
            return DiplomaticStance::Allied; // Same faction
        }

        self.relationships
            .get(&(faction1, faction2))
            .copied()
            .unwrap_or(DiplomaticStance::Neutral)
    }

    /// Check if two factions are allies
    pub fn are_allied(&self, faction1: FactionId, faction2: FactionId) -> bool {
        matches!(self.get_stance(faction1, faction2), DiplomaticStance::Allied)
    }

    /// Check if two factions are enemies
    pub fn are_enemies(&self, faction1: FactionId, faction2: FactionId) -> bool {
        matches!(self.get_stance(faction1, faction2), DiplomaticStance::Enemy)
    }

    /// Create a new alliance
    pub fn create_alliance(&mut self, name: String, founder: FactionId) -> u32 {
        let id = self.alliance_counter;
        self.alliance_counter += 1;

        let alliance = Alliance::new(id, name, founder);
        self.alliances.insert(id, alliance);

        id
    }

    /// Join an alliance
    pub fn join_alliance(&mut self, alliance_id: u32, faction: FactionId) -> bool {
        if let Some(alliance) = self.alliances.get_mut(&alliance_id) {
            alliance.add_member(faction);

            // Get list of members first to avoid double borrow
            let members: Vec<FactionId> = alliance.members.iter().copied().collect();

            // Update relationships with all alliance members
            for member in members {
                if member != faction {
                    self.set_stance(faction, member, DiplomaticStance::Allied);
                }
            }

            true
        } else {
            false
        }
    }

    /// Leave an alliance
    pub fn leave_alliance(&mut self, alliance_id: u32, faction: FactionId) -> bool {
        // Get members first before modifying
        let members_to_update: Option<Vec<FactionId>> = self.alliances.get(&alliance_id)
            .map(|a| a.members.iter().copied().collect());

        if let Some(members) = members_to_update {
            // Remove from alliance
            if let Some(alliance) = self.alliances.get_mut(&alliance_id) {
                alliance.remove_member(faction);
            }

            // Update relationships - no longer allied with former alliance members
            for member in members {
                if member != faction {
                    self.set_stance(faction, member, DiplomaticStance::Neutral);
                }
            }

            // Check if alliance should be dissolved
            let should_dissolve = self.alliances.get(&alliance_id)
                .map(|a| a.members.is_empty() || faction == a.founder)
                .unwrap_or(false);

            if should_dissolve {
                self.alliances.remove(&alliance_id);
            }

            true
        } else {
            false
        }
    }

    /// Get all allies of a faction
    pub fn get_allies(&self, faction: FactionId) -> Vec<FactionId> {
        let mut allies = Vec::new();

        for alliance in self.alliances.values() {
            if alliance.is_member(faction) {
                allies.extend(alliance.members.iter().filter(|&&f| f != faction));
            }
        }

        // Also check bilateral relationships
        for ((f1, f2), stance) in &self.relationships {
            if *f1 == faction && *stance == DiplomaticStance::Allied {
                allies.push(*f2);
            }
        }

        allies.sort();
        allies.dedup();
        allies
    }

    /// Get all enemies of a faction
    pub fn get_enemies(&self, faction: FactionId) -> Vec<FactionId> {
        let mut enemies = Vec::new();

        for ((f1, f2), stance) in &self.relationships {
            if *f1 == faction && *stance == DiplomaticStance::Enemy {
                enemies.push(*f2);
            }
        }

        enemies.sort();
        enemies.dedup();
        enemies
    }
}

/// Event for diplomatic changes
#[derive(Event, Debug, Clone)]
pub enum DiplomacyEvent {
    AllianceFormed { alliance_id: u32, name: String, founder: FactionId },
    AllianceJoined { alliance_id: u32, faction: FactionId },
    AllianceLeft { alliance_id: u32, faction: FactionId },
    StanceChanged { faction1: FactionId, faction2: FactionId, stance: DiplomaticStance },
}

/// Plugin for diplomacy system
pub struct DiplomacyPlugin;

impl Plugin for DiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiplomacyState>()
            .add_event::<DiplomacyEvent>();
    }
}