use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;
use crate::resources::ResourceType;

/// Technology tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TechTier {
    #[default]
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
    Tier4 = 4,
}

impl From<u32> for TechTier {
    fn from(tier: u32) -> Self {
        match tier {
            1 => TechTier::Tier1,
            2 => TechTier::Tier2,
            3 => TechTier::Tier3,
            4 => TechTier::Tier4,
            _ => TechTier::Tier1,
        }
    }
}

/// A technology that can be researched
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechId {
    // Tier 1 - Basic
    MiningEfficiency1,
    CarryCapacity1,
    MovementSpeed1,
    ConstructionSpeed1,

    // Tier 2 - Intermediate
    MiningEfficiency2,
    CarryCapacity2,
    MovementSpeed2,
    CombatDamage1,
    TowerRange1,

    // Tier 3 - Advanced
    MiningEfficiency3,
    CarryCapacity3,
    CombatDamage2,
    TowerRange2,
    SpawnSpeed1,

    // Tier 4 - Expert
    SuperMining,
    SuperCarry,
    SuperCombat,
    SuperTower,
}

/// Technology definition
#[derive(Debug, Clone)]
pub struct Tech {
    pub id: TechId,
    pub name: String,
    pub description: String,
    pub tier: u32,
    pub cost: HashMap<ResourceType, u32>,
    pub research_time: u32, // in ticks
    pub prerequisites: Vec<TechId>,
    pub effects: Vec<TechEffect>,
    pub unlocked_by_default: bool,
}

impl Tech {
    pub fn new(id: TechId, name: &str, tier: u32, research_time: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            tier,
            cost: HashMap::new(),
            research_time,
            prerequisites: Vec::new(),
            effects: Vec::new(),
            unlocked_by_default: false,
        }
    }

    pub fn with_cost(mut self, resource: ResourceType, amount: u32) -> Self {
        self.cost.insert(resource, amount);
        self
    }

    pub fn with_prerequisite(mut self, prereq: TechId) -> Self {
        self.prerequisites.push(prereq);
        self
    }

    pub fn with_effect(mut self, effect: TechEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn unlocked_by_default(mut self) -> Self {
        self.unlocked_by_default = true;
        self
    }
}

/// Effect of a technology
#[derive(Debug, Clone)]
pub enum TechEffect {
    MiningEfficiency(f32),
    CarryCapacity(f32),
    MovementSpeed(f32),
    CombatDamage(f32),
    TowerRange(f32),
    ConstructionSpeed(f32),
    SpawnSpeed(f32),
}

/// Research progress for a faction
#[derive(Debug, Clone, Component)]
pub struct ResearchProgress {
    pub faction_id: FactionId,
    pub completed_techs: HashMap<TechId, u64>, // tech_id -> tick completed
    pub current_research: Option<TechId>,
    pub research_progress: u32,
    pub research_target: u32,
}

impl ResearchProgress {
    pub fn new(faction_id: FactionId) -> Self {
        Self {
            faction_id,
            completed_techs: HashMap::new(),
            current_research: None,
            research_progress: 0,
            research_target: 0,
        }
    }

    pub fn has_tech(&self, tech_id: TechId) -> bool {
        self.completed_techs.contains_key(&tech_id)
    }

    pub fn start_research(&mut self, tech_id: TechId, research_time: u32) {
        self.current_research = Some(tech_id);
        self.research_progress = 0;
        self.research_target = research_time;
    }

    pub fn tick(&mut self) -> bool {
        if let Some(_) = self.current_research {
            self.research_progress += 1;
            self.research_progress >= self.research_target
        } else {
            false
        }
    }

    pub fn complete_research(&mut self, tick: u64) -> Option<TechId> {
        if let Some(tech_id) = self.current_research.take() {
            self.completed_techs.insert(tech_id, tick);
            self.research_progress = 0;
            self.research_target = 0;
            Some(tech_id)
        } else {
            None
        }
    }
}

/// Resource containing all tech definitions
#[derive(Resource, Debug, Clone)]
pub struct TechRegistry {
    techs: HashMap<TechId, Tech>,
}

impl Default for TechRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TechRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            techs: HashMap::new(),
        };
        registry.register_default_techs();
        registry
    }

    pub fn get(&self, id: TechId) -> Option<&Tech> {
        self.techs.get(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = &Tech> {
        self.techs.values()
    }

    pub fn get_techs_by_tier(&self, tier: TechTier) -> Vec<&Tech> {
        self.techs.values()
            .filter(|t| t.tier == tier as u32)
            .collect()
    }

    fn register_default_techs(&mut self) {
        // Tier 1
        self.register(Tech::new(TechId::MiningEfficiency1, "Mining Efficiency I", 1, 100)
            .with_cost(ResourceType::Power, 100)
            .with_effect(TechEffect::MiningEfficiency(0.1)));

        self.register(Tech::new(TechId::CarryCapacity1, "Carry Capacity I", 1, 100)
            .with_cost(ResourceType::Power, 100)
            .with_effect(TechEffect::CarryCapacity(0.1)));

        self.register(Tech::new(TechId::MovementSpeed1, "Movement Speed I", 1, 100)
            .with_cost(ResourceType::Power, 100)
            .with_effect(TechEffect::MovementSpeed(0.1)));

        self.register(Tech::new(TechId::ConstructionSpeed1, "Construction Speed I", 1, 100)
            .with_cost(ResourceType::Power, 100)
            .with_effect(TechEffect::ConstructionSpeed(0.1)));

        // Tier 2
        self.register(Tech::new(TechId::MiningEfficiency2, "Mining Efficiency II", 2, 200)
            .with_cost(ResourceType::Power, 200)
            .with_cost(ResourceType::Iron, 50)
            .with_prerequisite(TechId::MiningEfficiency1)
            .with_effect(TechEffect::MiningEfficiency(0.15)));

        self.register(Tech::new(TechId::CarryCapacity2, "Carry Capacity II", 2, 200)
            .with_cost(ResourceType::Power, 200)
            .with_cost(ResourceType::Iron, 50)
            .with_prerequisite(TechId::CarryCapacity1)
            .with_effect(TechEffect::CarryCapacity(0.15)));

        self.register(Tech::new(TechId::MovementSpeed2, "Movement Speed II", 2, 200)
            .with_cost(ResourceType::Power, 200)
            .with_cost(ResourceType::Iron, 50)
            .with_prerequisite(TechId::MovementSpeed1)
            .with_effect(TechEffect::MovementSpeed(0.15)));

        self.register(Tech::new(TechId::CombatDamage1, "Combat Damage I", 2, 200)
            .with_cost(ResourceType::Power, 200)
            .with_cost(ResourceType::Iron, 100)
            .with_effect(TechEffect::CombatDamage(0.1)));

        self.register(Tech::new(TechId::TowerRange1, "Tower Range I", 2, 200)
            .with_cost(ResourceType::Power, 200)
            .with_cost(ResourceType::Iron, 100)
            .with_effect(TechEffect::TowerRange(0.1)));

        // Tier 3
        self.register(Tech::new(TechId::MiningEfficiency3, "Mining Efficiency III", 3, 400)
            .with_cost(ResourceType::Power, 400)
            .with_cost(ResourceType::Iron, 100)
            .with_cost(ResourceType::Copper, 50)
            .with_prerequisite(TechId::MiningEfficiency2)
            .with_effect(TechEffect::MiningEfficiency(0.2)));

        self.register(Tech::new(TechId::CarryCapacity3, "Carry Capacity III", 3, 400)
            .with_cost(ResourceType::Power, 400)
            .with_cost(ResourceType::Iron, 100)
            .with_cost(ResourceType::Copper, 50)
            .with_prerequisite(TechId::CarryCapacity2)
            .with_effect(TechEffect::CarryCapacity(0.2)));

        self.register(Tech::new(TechId::CombatDamage2, "Combat Damage II", 3, 400)
            .with_cost(ResourceType::Power, 400)
            .with_cost(ResourceType::Iron, 200)
            .with_cost(ResourceType::Copper, 100)
            .with_prerequisite(TechId::CombatDamage1)
            .with_effect(TechEffect::CombatDamage(0.15)));

        self.register(Tech::new(TechId::TowerRange2, "Tower Range II", 3, 400)
            .with_cost(ResourceType::Power, 400)
            .with_cost(ResourceType::Iron, 200)
            .with_cost(ResourceType::Copper, 100)
            .with_prerequisite(TechId::TowerRange1)
            .with_effect(TechEffect::TowerRange(0.15)));

        self.register(Tech::new(TechId::SpawnSpeed1, "Spawn Speed I", 3, 400)
            .with_cost(ResourceType::Power, 400)
            .with_cost(ResourceType::Iron, 100)
            .with_cost(ResourceType::Copper, 100)
            .with_effect(TechEffect::SpawnSpeed(0.1)));

        // Tier 4
        self.register(Tech::new(TechId::SuperMining, "Super Mining", 4, 800)
            .with_cost(ResourceType::Power, 800)
            .with_cost(ResourceType::Iron, 200)
            .with_cost(ResourceType::Copper, 100)
            .with_cost(ResourceType::Silicon, 50)
            .with_prerequisite(TechId::MiningEfficiency3)
            .with_effect(TechEffect::MiningEfficiency(0.3)));

        self.register(Tech::new(TechId::SuperCarry, "Super Carry", 4, 800)
            .with_cost(ResourceType::Power, 800)
            .with_cost(ResourceType::Iron, 200)
            .with_cost(ResourceType::Copper, 100)
            .with_cost(ResourceType::Silicon, 50)
            .with_prerequisite(TechId::CarryCapacity3)
            .with_effect(TechEffect::CarryCapacity(0.3)));

        self.register(Tech::new(TechId::SuperCombat, "Super Combat", 4, 800)
            .with_cost(ResourceType::Power, 800)
            .with_cost(ResourceType::Iron, 300)
            .with_cost(ResourceType::Copper, 200)
            .with_cost(ResourceType::Silicon, 100)
            .with_prerequisite(TechId::CombatDamage2)
            .with_effect(TechEffect::CombatDamage(0.25)));

        self.register(Tech::new(TechId::SuperTower, "Super Tower", 4, 800)
            .with_cost(ResourceType::Power, 800)
            .with_cost(ResourceType::Iron, 300)
            .with_cost(ResourceType::Copper, 200)
            .with_cost(ResourceType::Silicon, 100)
            .with_prerequisite(TechId::TowerRange2)
            .with_effect(TechEffect::TowerRange(0.25)));
    }

    fn register(&mut self, tech: Tech) {
        self.techs.insert(tech.id, tech);
    }
}

/// Research Lab building component
#[derive(Debug, Clone, Component)]
pub struct ResearchLab {
    pub research_speed: f32,
}

impl Default for ResearchLab {
    fn default() -> Self {
        Self::new()
    }
}

impl ResearchLab {
    pub fn new() -> Self {
        Self {
            research_speed: 1.0,
        }
    }
}

/// System to process research progress
pub fn research_system(
    mut research_progress: Query<&mut ResearchProgress>,
    labs: Query<&ResearchLab>,
    tech_registry: Res<TechRegistry>,
    mut game_log: ResMut<crate::ui::GameLog>,
    tick: Res<crate::core::TickNumber>,
) {
    for mut progress in research_progress.iter_mut() {
        if progress.tick() {
            // Research complete
            if let Some(tech_id) = progress.complete_research(tick.0) {
                if let Some(tech) = tech_registry.get(tech_id) {
                    game_log.add(format!("Research complete: {}", tech.name));
                }
            }
        }
    }
}

/// Plugin for research system
pub struct ResearchPlugin;

impl Plugin for ResearchPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TechRegistry>();
    }
}