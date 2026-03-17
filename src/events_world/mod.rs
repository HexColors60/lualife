use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::core::TickNumber;
use crate::factions::FactionId;
use crate::world::WorldPos;
use crate::resources::ResourceType;

/// World event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEventType {
    MeteorStrike {
        position: WorldPos,
        radius: u32,
        damage: u32,
    },
    ResourceBoom {
        resource_type: ResourceType,
        position: WorldPos,
        amount: u32,
        duration_ticks: u64,
    },
    Earthquake {
        center: WorldPos,
        radius: u32,
        building_damage_percent: f32,
    },
    Flood {
        affected_area: (WorldPos, WorldPos), // min, max corners
        duration_ticks: u64,
    },
    VolcanicEruption {
        position: WorldPos,
        lava_radius: u32,
        ash_duration: u64,
    },
    ResourceDeposit {
        resource_type: ResourceType,
        position: WorldPos,
        amount: u32,
    },
    NpcInvasion {
        spawn_positions: Vec<WorldPos>,
        creep_count: u32,
        creep_tier: u32,
    },
    SeasonalChange {
        new_season: Season,
    },
}

/// Seasons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Season {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn resource_modifier(&self) -> f32 {
        match self {
            Season::Spring => 1.0,
            Season::Summer => 1.2,
            Season::Autumn => 0.9,
            Season::Winter => 0.6,
        }
    }

    pub fn movement_modifier(&self) -> f32 {
        match self {
            Season::Spring => 1.0,
            Season::Summer => 1.0,
            Season::Autumn => 0.9,
            Season::Winter => 0.7,
        }
    }

    pub fn power_consumption_modifier(&self) -> f32 {
        match self {
            Season::Spring => 1.0,
            Season::Summer => 0.9,
            Season::Autumn => 1.0,
            Season::Winter => 1.3,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }
}

/// Active world event
#[derive(Debug, Clone, Component)]
pub struct ActiveEvent {
    pub event_type: WorldEventType,
    pub start_tick: u64,
    pub end_tick: Option<u64>,
    pub affected_factions: Vec<FactionId>,
}

/// Event history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub event_type: WorldEventType,
    pub tick: u64,
    pub outcome: EventOutcome,
}

/// Outcome of an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventOutcome {
    Completed,
    Cancelled,
    Expired,
}

/// World event manager
#[derive(Debug, Clone, Resource, Default)]
pub struct WorldEventManager {
    pub active_events: Vec<ActiveEvent>,
    pub event_history: Vec<EventRecord>,
    pub current_season: Season,
    pub season_start_tick: u64,
    pub ticks_per_season: u64,
    pub event_cooldown: HashMap<WorldEventTypeKind, u64>,
}

/// Kind of world event for cooldown tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldEventTypeKind {
    MeteorStrike,
    ResourceBoom,
    Earthquake,
    Flood,
    VolcanicEruption,
    ResourceDeposit,
    NpcInvasion,
    SeasonalChange,
}

impl WorldEventManager {
    pub fn new() -> Self {
        Self {
            active_events: Vec::new(),
            event_history: Vec::new(),
            current_season: Season::Spring,
            season_start_tick: 0,
            ticks_per_season: 1000,
            event_cooldown: HashMap::new(),
        }
    }

    pub fn trigger_event(&mut self, event: WorldEventType, tick: u64) {
        let active_event = ActiveEvent {
            event_type: event.clone(),
            start_tick: tick,
            end_tick: None,
            affected_factions: Vec::new(),
        };
        self.active_events.push(active_event);

        // Set cooldown
        let kind = self.event_kind(&event);
        self.event_cooldown.insert(kind, tick + self.cooldown_duration(kind));
    }

    pub fn event_kind(&self, event: &WorldEventType) -> WorldEventTypeKind {
        match event {
            WorldEventType::MeteorStrike { .. } => WorldEventTypeKind::MeteorStrike,
            WorldEventType::ResourceBoom { .. } => WorldEventTypeKind::ResourceBoom,
            WorldEventType::Earthquake { .. } => WorldEventTypeKind::Earthquake,
            WorldEventType::Flood { .. } => WorldEventTypeKind::Flood,
            WorldEventType::VolcanicEruption { .. } => WorldEventTypeKind::VolcanicEruption,
            WorldEventType::ResourceDeposit { .. } => WorldEventTypeKind::ResourceDeposit,
            WorldEventType::NpcInvasion { .. } => WorldEventTypeKind::NpcInvasion,
            WorldEventType::SeasonalChange { .. } => WorldEventTypeKind::SeasonalChange,
        }
    }

    pub fn cooldown_duration(&self, kind: WorldEventTypeKind) -> u64 {
        match kind {
            WorldEventTypeKind::MeteorStrike => 500,
            WorldEventTypeKind::ResourceBoom => 200,
            WorldEventTypeKind::Earthquake => 800,
            WorldEventTypeKind::Flood => 600,
            WorldEventTypeKind::VolcanicEruption => 1000,
            WorldEventTypeKind::ResourceDeposit => 300,
            WorldEventTypeKind::NpcInvasion => 400,
            WorldEventTypeKind::SeasonalChange => 0,
        }
    }

    pub fn can_trigger(&self, kind: WorldEventTypeKind, tick: u64) -> bool {
        if let Some(&cooldown_end) = self.event_cooldown.get(&kind) {
            tick >= cooldown_end
        } else {
            true
        }
    }

    pub fn advance_season(&mut self, tick: u64) {
        if tick - self.season_start_tick >= self.ticks_per_season {
            self.current_season = self.current_season.next();
            self.season_start_tick = tick;
        }
    }
}

/// Event notification for UI
#[derive(Event, Debug, Clone)]
pub struct EventNotification {
    pub event_type: WorldEventType,
    pub message: String,
    pub severity: EventSeverity,
}

/// Event severity for UI display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventSeverity {
    Info,
    Warning,
    Critical,
}

/// System to process random world events
pub fn random_event_system(
    mut event_manager: ResMut<WorldEventManager>,
    mut notifications: EventWriter<EventNotification>,
    tick: Res<TickNumber>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Advance season
    let current_tick = tick.0;
    event_manager.advance_season(current_tick);

    // Random event chance (0.1% per tick)
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() < 0.001 {
        // Try to trigger a random event
        let event_types = [
            WorldEventTypeKind::MeteorStrike,
            WorldEventTypeKind::ResourceBoom,
            WorldEventTypeKind::ResourceDeposit,
        ];

        let kind = event_types[rng.gen_range(0..event_types.len())];
        if event_manager.can_trigger(kind, current_tick) {
            let event = generate_random_event(kind, &mut rng);
            let message = format_event_message(&event);
            
            notifications.send(EventNotification {
                event_type: event.clone(),
                message: message.clone(),
                severity: EventSeverity::Warning,
            });
            
            game_log.add(format!("World Event: {}", message));
            event_manager.trigger_event(event, current_tick);
        }
    }
}

fn generate_random_event(kind: WorldEventTypeKind, rng: &mut impl Rng) -> WorldEventType {
    match kind {
        WorldEventTypeKind::MeteorStrike => {
            WorldEventType::MeteorStrike {
                position: WorldPos::new(rng.gen_range(0..256), rng.gen_range(0..256)),
                radius: rng.gen_range(2..5),
                damage: rng.gen_range(50..150),
            }
        }
        WorldEventTypeKind::ResourceBoom => {
            WorldEventType::ResourceBoom {
                resource_type: ResourceType::Power,
                position: WorldPos::new(rng.gen_range(0..256), rng.gen_range(0..256)),
                amount: rng.gen_range(500..2000),
                duration_ticks: rng.gen_range(100..500),
            }
        }
        WorldEventTypeKind::ResourceDeposit => {
            WorldEventType::ResourceDeposit {
                resource_type: ResourceType::Iron,
                position: WorldPos::new(rng.gen_range(0..256), rng.gen_range(0..256)),
                amount: rng.gen_range(1000..5000),
            }
        }
        _ => WorldEventType::ResourceDeposit {
            resource_type: ResourceType::Power,
            position: WorldPos::new(128, 128),
            amount: 1000,
        }
    }
}

fn format_event_message(event: &WorldEventType) -> String {
    match event {
        WorldEventType::MeteorStrike { position, radius, .. } => {
            format!("Meteor strike at ({}, {}) with radius {}!", position.x, position.y, radius)
        }
        WorldEventType::ResourceBoom { resource_type, amount, .. } => {
            format!("Resource boom! {:?} +{} available!", resource_type, amount)
        }
        WorldEventType::Earthquake { center, radius, .. } => {
            format!("Earthquake at ({}, {}) radius {}!", center.x, center.y, radius)
        }
        WorldEventType::Flood { affected_area, .. } => {
            format!("Flood from ({}, {}) to ({}, {})!", 
                affected_area.0.x, affected_area.0.y,
                affected_area.1.x, affected_area.1.y)
        }
        WorldEventType::VolcanicEruption { position, lava_radius, .. } => {
            format!("Volcanic eruption at ({}, {}) with lava radius {}!", position.x, position.y, lava_radius)
        }
        WorldEventType::ResourceDeposit { resource_type, amount, position } => {
            format!("New {:?} deposit discovered at ({}, {}) with {} units!", 
                resource_type, position.x, position.y, amount)
        }
        WorldEventType::NpcInvasion { creep_count, .. } => {
            format!("NPC invasion with {} hostile creeps!", creep_count)
        }
        WorldEventType::SeasonalChange { new_season } => {
            format!("Season changed to {:?}!", new_season)
        }
    }
}

/// Plugin for world events
pub struct WorldEventsPlugin;

impl Plugin for WorldEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldEventManager>()
            .add_event::<EventNotification>()
            .add_systems(Update, random_event_system);
    }
}