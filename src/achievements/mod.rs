use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;
use crate::core::TickNumber;

/// Achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub category: AchievementCategory,
    pub requirement: AchievementRequirement,
    pub reward: AchievementReward,
    pub hidden: bool,
}

/// Achievement categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementCategory {
    Combat,
    Economy,
    Technology,
    Diplomacy,
    Exploration,
    Survival,
    Special,
}

/// Achievement requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRequirement {
    KillCreeps { count: u32 },
    KillBuildings { count: u32 },
    GatherResource { resource_type: u8, amount: u64 },
    BuildStructure { building_type: u8, count: u32 },
    ResearchTech { tech_id: u32 },
    SurviveTicks { ticks: u64 },
    ControlTerritory { percent: f32 },
    FormAlliance { count: u32 },
    WinGame { count: u32 },
    Custom { id: String, target: f32 },
}

/// Achievement rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    pub resource_bonus: Option<(u8, u32)>,
    pub stat_multiplier: Option<(String, f32)>,
    pub unlock_feature: Option<String>,
}

impl Default for AchievementReward {
    fn default() -> Self {
        Self {
            resource_bonus: None,
            stat_multiplier: None,
            unlock_feature: None,
        }
    }
}

/// Player's achievement progress
#[derive(Debug, Clone, Component, Default)]
pub struct AchievementProgress {
    pub unlocked: HashMap<String, u64>, // achievement_id -> unlock_tick
    pub progress: HashMap<String, f32>, // achievement_id -> progress (0.0-1.0)
}

impl AchievementProgress {
    pub fn is_unlocked(&self, achievement_id: &str) -> bool {
        self.unlocked.contains_key(achievement_id)
    }

    pub fn get_progress(&self, achievement_id: &str) -> f32 {
        self.progress.get(achievement_id).copied().unwrap_or(0.0)
    }

    pub fn unlock(&mut self, achievement_id: String, tick: u64) {
        self.unlocked.insert(achievement_id, tick);
    }

    pub fn update_progress(&mut self, achievement_id: String, progress: f32) {
        self.progress.insert(achievement_id, progress.clamp(0.0, 1.0));
    }
}

/// Achievement registry
#[derive(Debug, Clone, Resource, Default)]
pub struct AchievementRegistry {
    pub achievements: HashMap<String, Achievement>,
}

impl AchievementRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_default_achievements();
        registry
    }

    fn register_default_achievements(&mut self) {
        // Combat achievements
        self.register(Achievement {
            id: "first_blood".to_string(),
            name: "First Blood".to_string(),
            description: "Kill your first enemy creep".to_string(),
            icon: "sword".to_string(),
            category: AchievementCategory::Combat,
            requirement: AchievementRequirement::KillCreeps { count: 1 },
            reward: AchievementReward::default(),
            hidden: false,
        });

        self.register(Achievement {
            id: "warlord".to_string(),
            name: "Warlord".to_string(),
            description: "Kill 100 enemy creeps".to_string(),
            icon: "crossed_swords".to_string(),
            category: AchievementCategory::Combat,
            requirement: AchievementRequirement::KillCreeps { count: 100 },
            reward: AchievementReward {
                resource_bonus: Some((0, 1000)), // Power bonus
                ..default()
            },
            hidden: false,
        });

        // Economy achievements
        self.register(Achievement {
            id: "prospector".to_string(),
            name: "Prospector".to_string(),
            description: "Gather 10,000 Power".to_string(),
            icon: "gem".to_string(),
            category: AchievementCategory::Economy,
            requirement: AchievementRequirement::GatherResource { resource_type: 0, amount: 10000 },
            reward: AchievementReward::default(),
            hidden: false,
        });

        self.register(Achievement {
            id: "mogul".to_string(),
            name: "Mogul".to_string(),
            description: "Gather 100,000 Power".to_string(),
            icon: "coins".to_string(),
            category: AchievementCategory::Economy,
            requirement: AchievementRequirement::GatherResource { resource_type: 0, amount: 100000 },
            reward: AchievementReward {
                stat_multiplier: Some(("gather_rate".to_string(), 1.1)),
                ..default()
            },
            hidden: false,
        });

        // Technology achievements
        self.register(Achievement {
            id: "innovator".to_string(),
            name: "Innovator".to_string(),
            description: "Research your first technology".to_string(),
            icon: "lightbulb".to_string(),
            category: AchievementCategory::Technology,
            requirement: AchievementRequirement::ResearchTech { tech_id: 0 },
            reward: AchievementReward::default(),
            hidden: false,
        });

        // Survival achievements
        self.register(Achievement {
            id: "survivor".to_string(),
            name: "Survivor".to_string(),
            description: "Survive for 10,000 ticks".to_string(),
            icon: "heart".to_string(),
            category: AchievementCategory::Survival,
            requirement: AchievementRequirement::SurviveTicks { ticks: 10000 },
            reward: AchievementReward::default(),
            hidden: false,
        });

        // Diplomacy achievements
        self.register(Achievement {
            id: "diplomat".to_string(),
            name: "Diplomat".to_string(),
            description: "Form 5 alliances".to_string(),
            icon: "handshake".to_string(),
            category: AchievementCategory::Diplomacy,
            requirement: AchievementRequirement::FormAlliance { count: 5 },
            reward: AchievementReward::default(),
            hidden: false,
        });

        // Victory achievements
        self.register(Achievement {
            id: "conqueror".to_string(),
            name: "Conqueror".to_string(),
            description: "Win the game".to_string(),
            icon: "crown".to_string(),
            category: AchievementCategory::Special,
            requirement: AchievementRequirement::WinGame { count: 1 },
            reward: AchievementReward {
                unlock_feature: Some("victory_skin".to_string()),
                ..default()
            },
            hidden: false,
        });
    }

    pub fn register(&mut self, achievement: Achievement) {
        self.achievements.insert(achievement.id.clone(), achievement);
    }

    pub fn get(&self, id: &str) -> Option<&Achievement> {
        self.achievements.get(id)
    }

    pub fn by_category(&self, category: AchievementCategory) -> Vec<&Achievement> {
        self.achievements.values().filter(|a| a.category == category).collect()
    }
}

/// Player statistics
#[derive(Debug, Clone, Resource, Default, Serialize, Deserialize)]
pub struct PlayerStatistics {
    pub faction_id: FactionId,
    pub total_kills: u64,
    pub total_deaths: u64,
    pub resources_gathered: HashMap<u8, u64>,
    pub buildings_built: u64,
    pub buildings_lost: u64,
    pub techs_researched: u32,
    pub alliances_formed: u32,
    pub wars_fought: u32,
    pub games_won: u32,
    pub games_played: u32,
    pub total_ticks_survived: u64,
    pub max_territory_percent: f32,
    pub peak_creep_count: u32,
}

impl PlayerStatistics {
    pub fn new(faction_id: FactionId) -> Self {
        Self {
            faction_id,
            ..default()
        }
    }

    pub fn kd_ratio(&self) -> f32 {
        if self.total_deaths > 0 {
            self.total_kills as f32 / self.total_deaths as f32
        } else {
            self.total_kills as f32
        }
    }

    pub fn win_rate(&self) -> f32 {
        if self.games_played > 0 {
            self.games_won as f32 / self.games_played as f32
        } else {
            0.0
        }
    }

    pub fn record_kill(&mut self) {
        self.total_kills += 1;
    }

    pub fn record_death(&mut self) {
        self.total_deaths += 1;
    }

    pub fn record_resource(&mut self, resource_type: u8, amount: u64) {
        *self.resources_gathered.entry(resource_type).or_default() += amount;
    }

    pub fn record_building(&mut self) {
        self.buildings_built += 1;
    }

    pub fn record_tech(&mut self) {
        self.techs_researched += 1;
    }
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub faction_id: FactionId,
    pub faction_name: String,
    pub score: u64,
    pub wins: u32,
    pub games: u32,
}

/// Leaderboard types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeaderboardType {
    TotalKills,
    WinRate,
    ResourcesGathered,
    TerritoryControl,
    TechProgress,
}

/// Leaderboard manager
#[derive(Debug, Clone, Resource, Default)]
pub struct LeaderboardManager {
    pub leaderboards: HashMap<LeaderboardType, Vec<LeaderboardEntry>>,
}

impl LeaderboardManager {
    pub fn update(&mut self, leaderboard_type: LeaderboardType, entries: Vec<LeaderboardEntry>) {
        self.leaderboards.insert(leaderboard_type, entries);
    }

    pub fn get(&self, leaderboard_type: LeaderboardType) -> Option<&Vec<LeaderboardEntry>> {
        self.leaderboards.get(&leaderboard_type)
    }
}

/// Game history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    pub game_id: u64,
    pub start_tick: u64,
    pub end_tick: u64,
    pub winner: Option<FactionId>,
    pub participants: Vec<FactionId>,
    pub final_stats: HashMap<FactionId, PlayerStatistics>,
}

/// Game history manager
#[derive(Debug, Clone, Resource, Default)]
pub struct GameHistory {
    pub games: Vec<GameRecord>,
    pub current_game_id: u64,
}

impl GameHistory {
    pub fn new_game(&mut self) -> u64 {
        self.current_game_id += 1;
        self.current_game_id
    }

    pub fn record_game(&mut self, record: GameRecord) {
        self.games.push(record);
    }

    pub fn get_recent(&self, count: usize) -> Vec<&GameRecord> {
        self.games.iter().rev().take(count).collect()
    }
}

/// Achievement unlock event
#[derive(Event, Debug, Clone)]
pub struct AchievementUnlockEvent {
    pub faction_id: FactionId,
    pub achievement_id: String,
    pub achievement_name: String,
}

/// System to check achievement progress
pub fn achievement_check_system(
    registry: Res<AchievementRegistry>,
    stats: Res<PlayerStatistics>,
    mut progress: Query<&mut AchievementProgress>,
    mut events: EventWriter<AchievementUnlockEvent>,
    tick: Res<TickNumber>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for mut progress in progress.iter_mut() {
        for (id, achievement) in &registry.achievements {
            if progress.is_unlocked(id) {
                continue;
            }

            let current_progress = match &achievement.requirement {
                AchievementRequirement::KillCreeps { count } => {
                    (stats.total_kills as f32 / *count as f32).min(1.0)
                }
                AchievementRequirement::GatherResource { resource_type, amount } => {
                    let gathered = stats.resources_gathered.get(resource_type).copied().unwrap_or(0);
                    (gathered as f32 / *amount as f32).min(1.0)
                }
                AchievementRequirement::SurviveTicks { ticks } => {
                    (stats.total_ticks_survived as f32 / *ticks as f32).min(1.0)
                }
                _ => progress.get_progress(id),
            };

            progress.update_progress(id.clone(), current_progress);

            if current_progress >= 1.0 {
                progress.unlock(id.clone(), tick.0);
                events.send(AchievementUnlockEvent {
                    faction_id: stats.faction_id,
                    achievement_id: id.clone(),
                    achievement_name: achievement.name.clone(),
                });
                game_log.add(format!("Achievement Unlocked: {}!", achievement.name));
            }
        }
    }
}

/// Plugin for achievements system
pub struct AchievementsPlugin;

impl Plugin for AchievementsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AchievementRegistry>()
            .init_resource::<PlayerStatistics>()
            .init_resource::<LeaderboardManager>()
            .init_resource::<GameHistory>()
            .add_event::<AchievementUnlockEvent>()
            .add_systems(Update, achievement_check_system);
    }
}