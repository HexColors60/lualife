use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::factions::FactionId;

/// AI difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AiDifficulty {
    #[default]
    Easy,
    Normal,
    Hard,
    Nightmare,
}

impl AiDifficulty {
    pub fn decision_delay_ticks(&self) -> u32 {
        match self {
            AiDifficulty::Easy => 10,
            AiDifficulty::Normal => 5,
            AiDifficulty::Hard => 2,
            AiDifficulty::Nightmare => 1,
        }
    }

    pub fn resource_efficiency(&self) -> f32 {
        match self {
            AiDifficulty::Easy => 0.5,
            AiDifficulty::Normal => 0.75,
            AiDifficulty::Hard => 0.9,
            AiDifficulty::Nightmare => 1.0,
        }
    }

    pub fn aggression_factor(&self) -> f32 {
        match self {
            AiDifficulty::Easy => 0.3,
            AiDifficulty::Normal => 0.5,
            AiDifficulty::Hard => 0.7,
            AiDifficulty::Nightmare => 0.9,
        }
    }

    pub fn expansion_rate(&self) -> f32 {
        match self {
            AiDifficulty::Easy => 0.3,
            AiDifficulty::Normal => 0.5,
            AiDifficulty::Hard => 0.8,
            AiDifficulty::Nightmare => 1.0,
        }
    }
}

/// AI personality traits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AiPersonality {
    #[default]
    Balanced,
    Aggressive, // Focus on military and expansion
    Defensive,  // Focus on fortifications and defense
    Economic,   // Focus on resource gathering and trading
    Scientific, // Focus on research and tech
    Diplomatic, // Focus on alliances and trade agreements
}

impl AiPersonality {
    pub fn military_priority(&self) -> f32 {
        match self {
            AiPersonality::Aggressive => 0.8,
            AiPersonality::Defensive => 0.4,
            AiPersonality::Economic => 0.2,
            AiPersonality::Scientific => 0.3,
            AiPersonality::Diplomatic => 0.2,
            AiPersonality::Balanced => 0.5,
        }
    }

    pub fn economy_priority(&self) -> f32 {
        match self {
            AiPersonality::Aggressive => 0.3,
            AiPersonality::Defensive => 0.5,
            AiPersonality::Economic => 0.9,
            AiPersonality::Scientific => 0.4,
            AiPersonality::Diplomatic => 0.6,
            AiPersonality::Balanced => 0.5,
        }
    }

    pub fn research_priority(&self) -> f32 {
        match self {
            AiPersonality::Aggressive => 0.3,
            AiPersonality::Defensive => 0.4,
            AiPersonality::Economic => 0.3,
            AiPersonality::Scientific => 0.9,
            AiPersonality::Diplomatic => 0.4,
            AiPersonality::Balanced => 0.5,
        }
    }

    pub fn diplomacy_priority(&self) -> f32 {
        match self {
            AiPersonality::Aggressive => 0.1,
            AiPersonality::Defensive => 0.3,
            AiPersonality::Economic => 0.5,
            AiPersonality::Scientific => 0.3,
            AiPersonality::Diplomatic => 0.9,
            AiPersonality::Balanced => 0.5,
        }
    }
}

/// AI behavior configuration per faction
#[derive(Debug, Clone, Resource, Default)]
pub struct AiBehaviorConfig {
    pub faction_configs: HashMap<FactionId, FactionAiConfig>,
}

/// Per-faction AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionAiConfig {
    pub faction_id: FactionId,
    pub difficulty: AiDifficulty,
    pub personality: AiPersonality,
    pub retreat_threshold: f32,   // HP % below which retreat
    pub surrender_threshold: f32, // Territory % below which surrender
    pub expansion_desire: f32,    // How aggressively to expand
}

impl Default for FactionAiConfig {
    fn default() -> Self {
        Self {
            faction_id: FactionId(0),
            difficulty: AiDifficulty::Normal,
            personality: AiPersonality::Balanced,
            retreat_threshold: 0.2,
            surrender_threshold: 0.1,
            expansion_desire: 0.5,
        }
    }
}

impl FactionAiConfig {
    pub fn new(
        faction_id: FactionId,
        difficulty: AiDifficulty,
        personality: AiPersonality,
    ) -> Self {
        Self {
            faction_id,
            difficulty,
            personality,
            ..default()
        }
    }
}

/// AI learning data - tracks player behavior patterns
#[derive(Debug, Clone, Resource, Default)]
pub struct AiLearningData {
    pub player_attack_patterns: HashMap<FactionId, AttackPattern>,
    pub player_build_patterns: HashMap<FactionId, BuildPattern>,
    pub successful_strategies: HashMap<FactionId, Vec<StrategyRecord>>,
}

/// Tracked attack pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPattern {
    pub preferred_attack_direction: (i32, i32),
    pub average_attack_force_size: f32,
    pub attack_frequency: f32,
    pub preferred_target_types: Vec<u8>,
}

impl Default for AttackPattern {
    fn default() -> Self {
        Self {
            preferred_attack_direction: (0, 0),
            average_attack_force_size: 5.0,
            attack_frequency: 0.1,
            preferred_target_types: vec![],
        }
    }
}

/// Tracked build pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPattern {
    pub preferred_building_types: Vec<u8>,
    pub average_build_order: Vec<u8>,
    pub expansion_timing: f32,
}

impl Default for BuildPattern {
    fn default() -> Self {
        Self {
            preferred_building_types: vec![],
            average_build_order: vec![],
            expansion_timing: 100.0,
        }
    }
}

/// Record of a successful strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyRecord {
    pub strategy_type: StrategyType,
    pub success_rate: f32,
    pub use_count: u32,
    pub last_used_tick: u64,
}

/// Types of strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyType {
    Rush,
    Turtle,
    Expand,
    TechRush,
    EconomicBoom,
    DiplomaticVictory,
}

/// AI coordination between factions
#[derive(Debug, Clone, Resource, Default)]
pub struct AiCoordinator {
    pub alliances: HashMap<(FactionId, FactionId), AllianceType>,
    pub coordinated_attacks: Vec<CoordinatedAttack>,
    pub shared_intelligence: HashMap<FactionId, Vec<IntelligenceReport>>,
}

/// Types of alliances between AI factions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllianceType {
    None,
    NonAggression,
    Defensive,
    Offensive,
    FullAlliance,
}

/// A coordinated attack plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatedAttack {
    pub participants: Vec<FactionId>,
    pub target_faction: FactionId,
    pub attack_tick: u64,
    pub meeting_point: (i32, i32),
    pub status: AttackStatus,
}

/// Status of coordinated attack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttackStatus {
    Planning,
    Assembling,
    InProgress,
    Completed,
    Cancelled,
}

/// Intelligence report about another faction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceReport {
    pub about_faction: FactionId,
    pub reported_by: FactionId,
    pub report_tick: u64,
    pub intel_type: IntelType,
    pub reliability: f32,
}

/// Types of intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntelType {
    MilitaryStrength {
        estimated_units: u32,
    },
    ResourceStockpile {
        resource_type: u8,
        estimated_amount: u32,
    },
    BuildingLocation {
        building_type: u8,
        position: (i32, i32),
    },
    PlannedAttack {
        target_faction: FactionId,
        estimated_tick: u64,
    },
}

/// Plugin for advanced AI
pub struct AdvancedAiPlugin;

impl Plugin for AdvancedAiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiBehaviorConfig>()
            .init_resource::<AiLearningData>()
            .init_resource::<AiCoordinator>();
    }
}
