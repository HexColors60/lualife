//! AI faction coordination system for allied intelligence sharing.

use std::collections::{HashMap, HashSet};
use bevy::prelude::*;

use crate::creeps::Creep;
use crate::factions::FactionId;
use crate::diplomacy::DiplomacyState;

/// Shared intelligence about enemy positions
#[derive(Debug, Clone, Resource, Default)]
pub struct SharedIntel {
    /// Known enemy positions: faction -> set of (x, y) positions
    pub enemy_positions: HashMap<FactionId, HashSet<(i32, i32)>>,
    /// Last seen timestamp for each position
    pub last_seen: HashMap<(i32, i32), u64>,
    /// Threat level per faction (0.0 to 1.0)
    pub threat_levels: HashMap<FactionId, f32>,
    /// Shared attack targets
    pub priority_targets: HashMap<FactionId, Vec<(i32, i32)>>,
}

impl SharedIntel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Report an enemy sighting
    pub fn report_enemy(&mut self, faction: FactionId, x: i32, y: i32, tick: u64) {
        self.enemy_positions
            .entry(faction)
            .or_insert_with(HashSet::new)
            .insert((x, y));
        self.last_seen.insert((x, y), tick);
    }

    /// Get known enemy positions for a faction
    pub fn get_enemy_positions(&self, faction: FactionId) -> Option<&HashSet<(i32, i32)>> {
        self.enemy_positions.get(&faction)
    }

    /// Share intel between allied factions
    pub fn share_intel(&mut self, diplomacy: &DiplomacyState) {
        // Build alliance membership map
        let mut alliances: HashMap<FactionId, Vec<FactionId>> = HashMap::new();
        
        for (_, alliance) in diplomacy.get_alliances() {
            let members: Vec<FactionId> = alliance.members.iter().copied().collect();
            for &member in &members {
                let allies: Vec<FactionId> = members.iter().filter(|&&m| m != member).copied().collect();
                alliances.insert(member, allies);
            }
        }

        // Share enemy positions between allies
        let mut shared_positions: HashMap<FactionId, HashSet<(i32, i32)>> = HashMap::new();
        
        for (&faction, allies) in &alliances {
            let mut combined = HashSet::new();
            
            // Include own intel
            if let Some(positions) = self.enemy_positions.get(&faction) {
                combined.extend(positions.iter().cloned());
            }
            
            // Include allies' intel
            for &ally in allies {
                if let Some(positions) = self.enemy_positions.get(&ally) {
                    combined.extend(positions.iter().cloned());
                }
            }
            
            shared_positions.insert(faction, combined);
        }

        // Update with shared intel
        for (faction, positions) in shared_positions {
            self.enemy_positions.insert(faction, positions);
        }
    }

    /// Update threat levels based on enemy count
    pub fn update_threat_levels(&mut self) {
        for (&faction, positions) in &self.enemy_positions {
            let threat = (positions.len() as f32 / 100.0).min(1.0);
            self.threat_levels.insert(faction, threat);
        }
    }

    /// Propose a coordinated attack target
    pub fn propose_target(&mut self, faction: FactionId, target: (i32, i32)) {
        self.priority_targets
            .entry(faction)
            .or_insert_with(Vec::new)
            .push(target);
        
        // Keep only last 5 targets
        if let Some(targets) = self.priority_targets.get_mut(&faction) {
            if targets.len() > 5 {
                targets.remove(0);
            }
        }
    }
}

/// AI coordination message
#[derive(Debug, Clone, Event)]
pub enum CoordinationMessage {
    /// Request help at position
    RequestHelp { faction: FactionId, x: i32, y: i32, priority: f32 },
    /// Share enemy sighting
    EnemySighted { faction: FactionId, enemy_faction: FactionId, x: i32, y: i32, count: u32 },
    /// Propose joint attack
    ProposeAttack { faction: FactionId, target_faction: FactionId, x: i32, y: i32 },
    /// Accept alliance proposal
    AllianceAccepted { faction_a: FactionId, faction_b: FactionId },
    /// Coordinated attack starting
    AttackBeginning { factions: Vec<FactionId>, target_x: i32, target_y: i32 },
}

/// System to collect enemy sightings from creeps
pub fn collect_intel_system(
    creeps: Query<(&Creep, &Transform)>,
    mut intel: ResMut<SharedIntel>,
    diplomacy: Res<DiplomacyState>,
    time: Res<Time>,
    factions: Res<crate::factions::FactionRegistry>,
) {
    let tick = time.elapsed_seconds() as u64;
    
    for (creep, transform) in creeps.iter() {
        let x = transform.translation.x as i32;
        let y = transform.translation.y as i32;
        
        // Report own position as known enemy to hostile factions
        for faction in factions.all() {
            if diplomacy.are_enemies(faction.id, creep.faction_id) {
                intel.report_enemy(faction.id, x, y, tick);
            }
        }
    }
}

/// System to share intel between allied factions
pub fn share_intel_system(
    mut intel: ResMut<SharedIntel>,
    diplomacy: Res<DiplomacyState>,
) {
    intel.share_intel(&diplomacy);
    intel.update_threat_levels();
}

/// System to handle coordination messages
pub fn coordination_message_system(
    mut messages: EventReader<CoordinationMessage>,
    mut intel: ResMut<SharedIntel>,
) {
    for message in messages.read() {
        match message {
            CoordinationMessage::EnemySighted { faction, enemy_faction, x, y, count } => {
                // Multiple sightings increase priority
                for _ in 0..*count {
                    intel.report_enemy(*enemy_faction, *x, *y, 0);
                }
            }
            CoordinationMessage::ProposeAttack { faction, target_faction, x, y } => {
                intel.propose_target(*faction, (*x, *y));
            }
            _ => {}
        }
    }
}

/// Plugin for AI coordination system
pub struct AICoordinationPlugin;

impl Plugin for AICoordinationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SharedIntel>()
            .add_event::<CoordinationMessage>()
            .add_systems(Update, (
                collect_intel_system,
                share_intel_system.after(collect_intel_system),
                coordination_message_system,
            ));
    }
}