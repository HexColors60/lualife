use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::factions::FactionId;

/// Diplomatic stance between factions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiplomaticStance {
    Neutral, // Default - no special relationship
    Allied,  // Friendly - share vision, no combat
    Enemy,   // Hostile - can attack
    Truce,   // Temporary peace
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
    pub fn set_stance(
        &mut self,
        faction1: FactionId,
        faction2: FactionId,
        stance: DiplomaticStance,
    ) {
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
        matches!(
            self.get_stance(faction1, faction2),
            DiplomaticStance::Allied
        )
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
        let members_to_update: Option<Vec<FactionId>> = self
            .alliances
            .get(&alliance_id)
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
            let should_dissolve = self
                .alliances
                .get(&alliance_id)
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
    AllianceFormed {
        alliance_id: u32,
        name: String,
        founder: FactionId,
    },
    AllianceJoined {
        alliance_id: u32,
        faction: FactionId,
    },
    AllianceLeft {
        alliance_id: u32,
        faction: FactionId,
    },
    StanceChanged {
        faction1: FactionId,
        faction2: FactionId,
        stance: DiplomaticStance,
    },
}

/// Plugin for diplomacy system
pub struct DiplomacyPlugin;

impl Plugin for DiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiplomacyState>()
            .init_resource::<DiplomacyConfig>()
            .add_event::<DiplomacyEvent>()
            .add_systems(Update, (ai_diplomacy_decisions, ai_cooperation));
    }
}


/// Configuration for AI diplomacy behavior
#[derive(Resource, Debug, Clone)]
pub struct DiplomacyConfig {
    pub min_alliance_power_ratio: f32,
    pub max_enemies_before_alliance: usize,
    pub diplomacy_check_interval: u64,
    pub alliance_proposal_chance: f32,
    pub war_declaration_chance: f32,
}

impl Default for DiplomacyConfig {
    fn default() -> Self {
        Self {
            min_alliance_power_ratio: 0.8,
            max_enemies_before_alliance: 3,
            diplomacy_check_interval: 500,
            alliance_proposal_chance: 0.1,
            war_declaration_chance: 0.05,
        }
    }
}

/// AI diplomacy decisions
fn ai_diplomacy_decisions(
    mut diplomacy: ResMut<DiplomacyState>,
    config: Res<DiplomacyConfig>,
    tick: Res<crate::core::TickNumber>,
    creeps: Query<&crate::creeps::Creep>,
    mut events: EventWriter<DiplomacyEvent>,
) {
    if tick.0 % config.diplomacy_check_interval != 0 {
        return;
    }

    let mut faction_power: std::collections::HashMap<crate::factions::FactionId, u32> = 
        std::collections::HashMap::new();
    
    for creep in creeps.iter() {
        *faction_power.entry(creep.faction_id).or_insert(0) += 1;
    }

    let all_factions: Vec<crate::factions::FactionId> = faction_power.keys().copied().collect();

    for &faction_id in &all_factions {
        let my_power = faction_power.get(&faction_id).copied().unwrap_or(0);
        let my_enemies = diplomacy.get_enemies(faction_id);

        if my_enemies.len() >= config.max_enemies_before_alliance || my_power < 3 {
            for &other_faction in &all_factions {
                if other_faction == faction_id { continue; }
                let stance = diplomacy.get_stance(faction_id, other_faction);
                if stance != DiplomaticStance::Neutral { continue; }

                let other_power = faction_power.get(&other_faction).copied().unwrap_or(0);
                let ratio = if my_power > 0 { other_power as f32 / my_power as f32 } else { 1.0 };

                if ratio >= config.min_alliance_power_ratio && rand::random::<f32>() < config.alliance_proposal_chance {
                    diplomacy.set_stance(faction_id, other_faction, DiplomaticStance::Allied);
                    events.send(DiplomacyEvent::StanceChanged {
                        faction1: faction_id, faction2: other_faction, stance: DiplomaticStance::Allied
                    });
                }
            }
        }

        if my_power > 5 && my_enemies.len() < 2 && rand::random::<f32>() < config.war_declaration_chance {
            for &other_faction in &all_factions {
                if other_faction == faction_id { continue; }
                let stance = diplomacy.get_stance(faction_id, other_faction);
                if stance != DiplomaticStance::Neutral { continue; }

                let other_power = faction_power.get(&other_faction).copied().unwrap_or(0);
                if other_power > 0 && my_power > other_power * 2 {
                    diplomacy.set_stance(faction_id, other_faction, DiplomaticStance::Enemy);
                    events.send(DiplomacyEvent::StanceChanged {
                        faction1: faction_id, faction2: other_faction, stance: DiplomaticStance::Enemy
                    });
                    break;
                }
            }
        }
    }
}


/// AI cooperation - allied factions share intelligence and coordinate
fn ai_cooperation(
    diplomacy: Res<DiplomacyState>,
    tick: Res<crate::core::TickNumber>,
    mut events: EventWriter<DiplomacyEvent>,
) {
    // Only check periodically
    if tick.0 % 1000 != 0 {
        return;
    }

    // For each faction, check if allies share common enemies
    for faction_id in 0..32u16 {
        let faction = crate::factions::FactionId(faction_id);
        let allies = diplomacy.get_allies(faction);
        let enemies = diplomacy.get_enemies(faction);

        // If we have allies and enemies, they should also consider our enemies as theirs
        if !allies.is_empty() && !enemies.is_empty() {
            for &ally in &allies {
                for &enemy in &enemies {
                    let ally_stance = diplomacy.get_stance(ally, enemy);
                    // If ally is neutral to our enemy, encourage them to become hostile
                    if ally_stance == DiplomaticStance::Neutral && rand::random::<f32>() < 0.1 {
                        // 10% chance to influence ally's stance
                        // This is just an event notification - actual stance change would need mutable diplomacy
                        events.send(DiplomacyEvent::StanceChanged {
                            faction1: ally,
                            faction2: enemy,
                            stance: DiplomaticStance::Enemy,
                        });
                    }
                }
            }
        }
    }
}