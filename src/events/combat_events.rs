use bevy::prelude::*;

use crate::factions::FactionId;

#[derive(Event, Debug, Clone)]
pub enum CombatEvent {
    AttackPerformed {
        attacker: Entity,
        attacker_faction: FactionId,
        defender: Entity,
        defender_faction: FactionId,
        damage: f32,
    },
    UnitKilled {
        killer: Option<Entity>,
        killer_faction: Option<FactionId>,
        victim: Entity,
        victim_faction: FactionId,
    },
    BuildingDestroyed {
        attacker: Option<Entity>,
        building: Entity,
        building_faction: FactionId,
    },
}