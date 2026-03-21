//! Quest/mission system for objectives and rewards.
//!
//! Provides dynamic quest generation, tracking, and completion rewards.

mod ui;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::TickNumber;
use crate::factions::FactionId;
use crate::resources::ResourceType;

pub use ui::*;

/// Quest types with different objectives
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuestType {
    /// Kill a number of enemy units
    Elimination,
    /// Gather a specific resource amount
    Gathering,
    /// Build structures
    Construction,
    /// Explore rooms
    Exploration,
    /// Defend a location for a duration
    Defense,
    /// Survive for a number of ticks
    Survival,
    /// Control a number of rooms
    Territory,
    /// Reach a faction power level
    Power,
    /// Complete within time limit
    SpeedRun,
    /// Chain of objectives
    Chain,
}

/// Quest objective definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub objective_type: QuestObjectiveType,
    pub current_progress: u32,
    pub target_amount: u32,
    pub is_completed: bool,
}

/// Types of quest objectives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestObjectiveType {
    KillUnits { enemy_faction: Option<FactionId> },
    GatherResource { resource_type: ResourceType },
    BuildStructure { building_type: String },
    ExploreRooms { count: u32 },
    DefendPosition { x: u32, y: u32, duration_ticks: u64 },
    Survive { ticks: u64 },
    ControlRooms { count: u32 },
    ReachPower { amount: u32 },
    Custom { description: String },
}

impl QuestObjective {
    pub fn new(objective_type: QuestObjectiveType, target: u32) -> Self {
        Self {
            objective_type,
            current_progress: 0,
            target_amount: target,
            is_completed: false,
        }
    }

    pub fn add_progress(&mut self, amount: u32) -> bool {
        if self.is_completed {
            return true;
        }
        self.current_progress = (self.current_progress + amount).min(self.target_amount);
        if self.current_progress >= self.target_amount {
            self.is_completed = true;
        }
        self.is_completed
    }

    pub fn progress_percent(&self) -> f32 {
        if self.target_amount == 0 {
            return 100.0;
        }
        (self.current_progress as f32 / self.target_amount as f32) * 100.0
    }
}

/// Quest difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum QuestDifficulty {
    #[default]
    Trivial,
    Easy,
    Normal,
    Hard,
    Extreme,
    Legendary,
}

impl QuestDifficulty {
    pub fn reward_multiplier(&self) -> f32 {
        match self {
            QuestDifficulty::Trivial => 0.5,
            QuestDifficulty::Easy => 0.75,
            QuestDifficulty::Normal => 1.0,
            QuestDifficulty::Hard => 1.5,
            QuestDifficulty::Extreme => 2.0,
            QuestDifficulty::Legendary => 3.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            QuestDifficulty::Trivial => "Trivial",
            QuestDifficulty::Easy => "Easy",
            QuestDifficulty::Normal => "Normal",
            QuestDifficulty::Hard => "Hard",
            QuestDifficulty::Extreme => "Extreme",
            QuestDifficulty::Legendary => "Legendary",
        }
    }
}

/// Quest rewards
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuestReward {
    pub resources: HashMap<ResourceType, u32>,
    pub experience: u32,
    pub unlocks: Vec<String>,
}

impl QuestReward {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_resource(mut self, resource: ResourceType, amount: u32) -> Self {
        self.resources.insert(resource, amount);
        self
    }

    pub fn with_experience(mut self, amount: u32) -> Self {
        self.experience = amount;
        self
    }

    pub fn with_unlock(mut self, unlock: &str) -> Self {
        self.unlocks.push(unlock.to_string());
        self
    }
}

/// Quest definition
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub quest_type: QuestType,
    pub difficulty: QuestDifficulty,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestReward,
    pub time_limit: Option<u64>, // Ticks
    pub start_tick: u64,
    pub is_active: bool,
    pub is_completed: bool,
    pub is_failed: bool,
    pub faction_id: Option<FactionId>, // None = global quest
}

impl Quest {
    pub fn new(id: u32, name: &str, quest_type: QuestType) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            quest_type,
            difficulty: QuestDifficulty::Normal,
            objectives: Vec::new(),
            rewards: QuestReward::default(),
            time_limit: None,
            start_tick: 0,
            is_active: false,
            is_completed: false,
            is_failed: false,
            faction_id: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_difficulty(mut self, difficulty: QuestDifficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn with_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    pub fn with_reward(mut self, reward: QuestReward) -> Self {
        self.rewards = reward;
        self
    }

    pub fn with_time_limit(mut self, ticks: u64) -> Self {
        self.time_limit = Some(ticks);
        self
    }

    pub fn for_faction(mut self, faction_id: FactionId) -> Self {
        self.faction_id = Some(faction_id);
        self
    }

    pub fn activate(&mut self, tick: u64) {
        self.start_tick = tick;
        self.is_active = true;
    }

    pub fn check_completion(&mut self) -> bool {
        if self.is_completed || self.is_failed {
            return self.is_completed;
        }

        // Check if all objectives are completed
        self.is_completed = self.objectives.iter().all(|o| o.is_completed);
        self.is_completed
    }

    pub fn check_failure(&mut self, current_tick: u64) -> bool {
        if self.is_completed || self.is_failed {
            return self.is_failed;
        }

        // Check time limit
        if let Some(limit) = self.time_limit {
            if current_tick - self.start_tick > limit {
                self.is_failed = true;
                self.is_active = false;
                return true;
            }
        }

        false
    }

    pub fn progress_percent(&self) -> f32 {
        if self.objectives.is_empty() {
            return 100.0;
        }
        let total: f32 = self.objectives.iter().map(|o| o.progress_percent()).sum();
        total / self.objectives.len() as f32
    }

    pub fn remaining_ticks(&self, current_tick: u64) -> Option<u64> {
        self.time_limit.map(|limit| {
            let elapsed = current_tick - self.start_tick;
            if elapsed >= limit { 0 } else { limit - elapsed }
        })
    }
}

/// Quest registry for managing all quests
#[derive(Resource, Debug, Clone, Default)]
pub struct QuestRegistry {
    pub quests: HashMap<u32, Quest>,
    pub next_id: u32,
    pub active_quests: Vec<u32>,
    pub completed_count: u32,
}

impl QuestRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_quest(&mut self, mut quest: Quest) -> u32 {
        let id = self.next_id;
        quest.id = id;
        self.quests.insert(id, quest);
        self.next_id += 1;
        id
    }

    pub fn get_quest(&self, id: u32) -> Option<&Quest> {
        self.quests.get(&id)
    }

    pub fn get_quest_mut(&mut self, id: u32) -> Option<&mut Quest> {
        self.quests.get_mut(&id)
    }

    pub fn activate_quest(&mut self, id: u32, tick: u64) {
        if let Some(quest) = self.quests.get_mut(&id) {
            quest.activate(tick);
            self.active_quests.push(id);
        }
    }

    pub fn complete_quest(&mut self, id: u32) -> Option<QuestReward> {
        if let Some(quest) = self.quests.get_mut(&id) {
            quest.is_completed = true;
            quest.is_active = false;
            self.completed_count += 1;
            self.active_quests.retain(|&qid| qid != id);
            Some(quest.rewards.clone())
        } else {
            None
        }
    }

    pub fn fail_quest(&mut self, id: u32) {
        if let Some(quest) = self.quests.get_mut(&id) {
            quest.is_failed = true;
            quest.is_active = false;
            self.active_quests.retain(|&qid| qid != id);
        }
    }

    pub fn get_active_quests(&self) -> Vec<&Quest> {
        self.active_quests
            .iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }

    pub fn get_completed_quests(&self) -> Vec<&Quest> {
        self.quests
            .values()
            .filter(|q| q.is_completed)
            .collect()
    }
}

/// Quest event - when a quest is completed
#[derive(Event, Debug, Clone)]
pub struct QuestCompletedEvent {
    pub quest_id: u32,
    pub quest_name: String,
    pub rewards: QuestReward,
}

/// Quest event - when a quest fails
#[derive(Event, Debug, Clone)]
pub struct QuestFailedEvent {
    pub quest_id: u32,
    pub quest_name: String,
    pub reason: String,
}

/// Quest event - when progress is made
#[derive(Event, Debug, Clone)]
pub struct QuestProgressEvent {
    pub quest_id: u32,
    pub objective_index: usize,
    pub progress: u32,
    pub target: u32,
}

/// System to check quest completion
pub fn quest_completion_system(
    mut registry: ResMut<QuestRegistry>,
    mut completed_events: EventWriter<QuestCompletedEvent>,
    mut failed_events: EventWriter<QuestFailedEvent>,
    tick: Res<TickNumber>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    let current_tick = tick.0;
    let active: Vec<u32> = registry.active_quests.clone();

    for quest_id in active {
        if let Some(quest) = registry.get_quest_mut(quest_id) {
            // Check for failure
            if quest.check_failure(current_tick) {
                game_log.add(format!("❌ Quest '{}' failed!", quest.name));

                failed_events.send(QuestFailedEvent {
                    quest_id,
                    quest_name: quest.name.clone(),
                    reason: "Time limit exceeded".to_string(),
                });
                continue;
            }

            // Check for completion
            if quest.check_completion() {
                let quest_name = quest.name.clone();
                let rewards = quest.rewards.clone();

                game_log.add(format!("✅ Quest '{}' completed!", quest_name));

                completed_events.send(QuestCompletedEvent {
                    quest_id,
                    quest_name,
                    rewards,
                });
            }
        }
    }
}

/// System to apply quest rewards
pub fn quest_reward_system(
    mut events: EventReader<QuestCompletedEvent>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for event in events.read() {
        // Log rewards
        if !event.rewards.resources.is_empty() {
            let resources: Vec<String> = event.rewards
                .resources
                .iter()
                .map(|(r, a)| format!("{:?} x{}", r, a))
                .collect();
            game_log.add(format!("🎁 Rewards: {}", resources.join(", ")));
        }

        if event.rewards.experience > 0 {
            game_log.add(format!("⭐ Experience: +{}", event.rewards.experience));
        }

        for unlock in &event.rewards.unlocks {
            game_log.add(format!("🔓 Unlocked: {}", unlock));
        }
    }
}

/// Generate random quests
pub fn generate_random_quests(
    mut registry: ResMut<QuestRegistry>,
    tick: Res<TickNumber>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Only generate occasionally
    if tick.0 % 1000 != 0 || tick.0 == 0 {
        return;
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Generate a random quest
    let quest_types = [
        ("Gather Power", QuestType::Gathering, QuestDifficulty::Easy),
        ("Eliminate Threats", QuestType::Elimination, QuestDifficulty::Normal),
        ("Expand Territory", QuestType::Territory, QuestDifficulty::Hard),
        ("Survive the Storm", QuestType::Survival, QuestDifficulty::Normal),
    ];

    let (name, qtype, difficulty) = quest_types[rng.gen_range(0..quest_types.len())];

    let quest = Quest::new(registry.next_id, name, qtype)
        .with_difficulty(difficulty)
        .with_description(&format!("{} difficulty quest", difficulty.name()))
        .with_objective(QuestObjective::new(
            QuestObjectiveType::GatherResource { resource_type: ResourceType::Power },
            500,
        ))
        .with_reward(
            QuestReward::new()
                .with_resource(ResourceType::Power, 200)
                .with_experience(100)
        );

    let id = registry.add_quest(quest);
    registry.activate_quest(id, tick.0);

    game_log.add(format!("📜 New quest available: {}", name));
}

/// Plugin for quest system
pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuestRegistry>()
            .add_event::<QuestCompletedEvent>()
            .add_event::<QuestFailedEvent>()
            .add_event::<QuestProgressEvent>()
            .add_systems(
                Update,
                (
                    quest_completion_system,
                    quest_reward_system,
                    generate_random_quests,
                ),
            );
    }
}