use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::factions::FactionId;

/// Reputation level with a faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReputationLevel {
    Hostile,    // -100 to -50
    Unfriendly, // -50 to -10
    Neutral,    // -10 to 10
    Friendly,   // 10 to 50
    Allied,     // 50 to 100
}

impl Default for ReputationLevel {
    fn default() -> Self {
        Self::Neutral
    }
}

impl ReputationLevel {
    pub fn from_value(value: i32) -> Self {
        if value >= 50 {
            Self::Allied
        } else if value >= 10 {
            Self::Friendly
        } else if value >= -10 {
            Self::Neutral
        } else if value >= -50 {
            Self::Unfriendly
        } else {
            Self::Hostile
        }
    }

    pub fn color(&self) -> bevy::prelude::Color {
        match self {
            Self::Hostile => bevy::prelude::Color::srgb(1.0, 0.0, 0.0),
            Self::Unfriendly => bevy::prelude::Color::srgb(1.0, 0.5, 0.0),
            Self::Neutral => bevy::prelude::Color::srgb(0.8, 0.8, 0.8),
            Self::Friendly => bevy::prelude::Color::srgb(0.5, 1.0, 0.5),
            Self::Allied => bevy::prelude::Color::srgb(0.0, 1.0, 0.0),
        }
    }
}

/// Reputation between two factions
#[derive(Debug, Clone)]
pub struct Reputation {
    pub faction1: FactionId,
    pub faction2: FactionId,
    pub value: i32, // -100 to 100
}

impl Reputation {
    pub fn new(faction1: FactionId, faction2: FactionId) -> Self {
        Self {
            faction1,
            faction2,
            value: 0, // Start neutral
        }
    }

    pub fn level(&self) -> ReputationLevel {
        ReputationLevel::from_value(self.value)
    }

    pub fn modify(&mut self, delta: i32) {
        self.value = (self.value + delta).clamp(-100, 100);
    }

    pub fn set(&mut self, value: i32) {
        self.value = value.clamp(-100, 100);
    }
}

/// Reputation manager for all factions
#[derive(Resource, Debug, Clone, Default)]
pub struct ReputationManager {
    /// Reputation values: (faction1, faction2) -> Reputation
    /// Always stored with lower faction ID first
    reputations: HashMap<(FactionId, FactionId), Reputation>,
}

impl ReputationManager {
    pub fn new() -> Self {
        Self::default()
    }

    fn make_key(faction1: FactionId, faction2: FactionId) -> (FactionId, FactionId) {
        if faction1 < faction2 {
            (faction1, faction2)
        } else {
            (faction2, faction1)
        }
    }

    pub fn get_reputation(&self, faction1: FactionId, faction2: FactionId) -> i32 {
        if faction1 == faction2 {
            return 100; // Same faction is always allied
        }

        let key = Self::make_key(faction1, faction2);
        self.reputations.get(&key).map(|r| r.value).unwrap_or(0)
    }

    pub fn get_reputation_level(
        &self,
        faction1: FactionId,
        faction2: FactionId,
    ) -> ReputationLevel {
        ReputationLevel::from_value(self.get_reputation(faction1, faction2))
    }

    pub fn modify_reputation(&mut self, faction1: FactionId, faction2: FactionId, delta: i32) {
        if faction1 == faction2 {
            return;
        }

        let key = Self::make_key(faction1, faction2);

        let reputation = self
            .reputations
            .entry(key)
            .or_insert_with(|| Reputation::new(key.0, key.1));

        reputation.modify(delta);
    }

    pub fn set_reputation(&mut self, faction1: FactionId, faction2: FactionId, value: i32) {
        if faction1 == faction2 {
            return;
        }

        let key = Self::make_key(faction1, faction2);

        let reputation = self
            .reputations
            .entry(key)
            .or_insert_with(|| Reputation::new(key.0, key.1));

        reputation.set(value);
    }

    pub fn get_all_reputations(&self, faction: FactionId) -> Vec<(FactionId, i32)> {
        self.reputations
            .values()
            .filter_map(|r| {
                if r.faction1 == faction {
                    Some((r.faction2, r.value))
                } else if r.faction2 == faction {
                    Some((r.faction1, r.value))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Events that affect reputation
#[derive(Event, Debug, Clone)]
pub enum ReputationEvent {
    Attacked {
        attacker: FactionId,
        victim: FactionId,
    },
    Traded {
        faction1: FactionId,
        faction2: FactionId,
    },
    Allied {
        faction1: FactionId,
        faction2: FactionId,
    },
    Betrayed {
        traitor: FactionId,
        victim: FactionId,
    },
    Helped {
        helper: FactionId,
        helped: FactionId,
    },
}

/// System to process reputation events
pub fn reputation_event_system(
    mut events: EventReader<ReputationEvent>,
    mut reputation_manager: ResMut<ReputationManager>,
) {
    for event in events.read() {
        match event {
            ReputationEvent::Attacked { attacker, victim } => {
                reputation_manager.modify_reputation(*attacker, *victim, -20);
            }
            ReputationEvent::Traded { faction1, faction2 } => {
                reputation_manager.modify_reputation(*faction1, *faction2, 5);
            }
            ReputationEvent::Allied { faction1, faction2 } => {
                reputation_manager.set_reputation(*faction1, *faction2, 75);
            }
            ReputationEvent::Betrayed { traitor, victim } => {
                reputation_manager.modify_reputation(*traitor, *victim, -50);
            }
            ReputationEvent::Helped { helper, helped } => {
                reputation_manager.modify_reputation(*helper, *helped, 10);
            }
        }
    }
}

/// Plugin for reputation system
pub struct ReputationPlugin;

impl Plugin for ReputationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReputationManager>()
            .add_event::<ReputationEvent>()
            .add_systems(Update, reputation_event_system);
    }
}
