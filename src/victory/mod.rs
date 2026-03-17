use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;
use crate::territory::TerritoryManager;
use crate::creeps::Creep;
use crate::buildings::Building;

/// Victory condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VictoryCondition {
    Domination,      // Control X% of the map
    Elimination,     // Eliminate all other factions
    Economic,        // Accumulate X resources
    Technological,   // Research all techs
    Alliance,        // Form an alliance with X factions
}

impl Default for VictoryCondition {
    fn default() -> Self {
        Self::Domination
    }
}

/// Victory progress for a faction
#[derive(Debug, Clone, Default)]
pub struct VictoryProgress {
    pub domination_score: f32,
    pub elimination_score: f32,
    pub economic_score: f32,
    pub tech_score: f32,
    pub alliance_score: f32,
}

impl VictoryProgress {
    pub fn get_score(&self, condition: VictoryCondition) -> f32 {
        match condition {
            VictoryCondition::Domination => self.domination_score,
            VictoryCondition::Elimination => self.elimination_score,
            VictoryCondition::Economic => self.economic_score,
            VictoryCondition::Technological => self.tech_score,
            VictoryCondition::Alliance => self.alliance_score,
        }
    }
}

/// Victory state for the game
#[derive(Resource, Debug, Clone, Default)]
pub struct VictoryState {
    pub condition: VictoryCondition,
    pub threshold: f32,
    pub progress: HashMap<FactionId, VictoryProgress>,
    pub winner: Option<FactionId>,
    pub game_over: bool,
}

impl VictoryState {
    pub fn new(condition: VictoryCondition, threshold: f32) -> Self {
        Self {
            condition,
            threshold,
            progress: HashMap::new(),
            winner: None,
            game_over: false,
        }
    }

    pub fn get_progress_mut(&mut self, faction: FactionId) -> &mut VictoryProgress {
        self.progress.entry(faction).or_default()
    }

    pub fn check_victory(&mut self) -> Option<FactionId> {
        if self.game_over {
            return self.winner;
        }

        for (faction, progress) in &self.progress {
            if progress.get_score(self.condition) >= self.threshold {
                self.winner = Some(*faction);
                self.game_over = true;
                return self.winner;
            }
        }

        None
    }
}

/// System to calculate victory progress
pub fn victory_progress_system(
    mut victory_state: ResMut<VictoryState>,
    territory_manager: Res<TerritoryManager>,
    creeps: Query<&Creep>,
    buildings: Query<&Building>,
    factions: Res<crate::factions::FactionRegistry>,
) {
    let total_rooms = 32 * 32; // 1024 rooms

    for faction in factions.all() {
        let progress = victory_state.get_progress_mut(faction.id);

        // Domination score: percentage of rooms controlled
        let territory_count = territory_manager.get_territory_count(faction.id);
        progress.domination_score = territory_count as f32 / total_rooms as f32;

        // Elimination score: based on surviving factions
        let creep_count = creeps.iter().filter(|c| c.faction_id == faction.id).count();
        let building_count = buildings.iter().filter(|b| b.faction_id == faction.id).count();
        progress.elimination_score = if creep_count > 0 || building_count > 0 { 1.0 } else { 0.0 };

        // Economic score: based on resources (placeholder)
        progress.economic_score = 0.0;

        // Tech score: based on research progress (placeholder)
        progress.tech_score = 0.0;

        // Alliance score: based on alliance size (placeholder)
        progress.alliance_score = 0.0;
    }

    // Check for victory
    if let Some(winner) = victory_state.check_victory() {
        tracing::info!("Faction {} has won the game!", winner.0);
    }
}

/// System to display victory status
pub fn victory_display_system(
    victory_state: Res<VictoryState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if victory_state.is_changed() && victory_state.game_over {
        if let Some(winner) = victory_state.winner {
            game_log.add("=== VICTORY! ===".to_string());
            game_log.add(format!("Faction {} has won!", winner.0));
            game_log.add(format!("Condition: {:?}", victory_state.condition));
        }
    }
}

/// Plugin for victory system
pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VictoryState::new(VictoryCondition::Domination, 0.5))
            .add_systems(Update, (
                victory_progress_system,
                victory_display_system,
            ));
    }
}