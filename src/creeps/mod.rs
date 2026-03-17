mod actions;
mod bundle;
mod combat;
mod components;
mod inventory;
mod movement;
mod role;
mod spawning;
mod stats;
mod upkeep;

pub use actions::*;
pub use bundle::*;
pub use combat::*;
pub use components::*;
pub use inventory::*;
pub use movement::*;
pub use role::*;
pub use spawning::*;
pub use stats::*;
pub use upkeep::*;

use bevy::prelude::*;

pub struct CreepsPlugin;

impl Plugin for CreepsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreepEvent>();
    }
}

#[derive(Event, Debug, Clone)]
pub enum CreepEvent {
    CreepSpawned { entity: Entity, faction_id: crate::factions::FactionId },
    CreepDied { entity: Entity, faction_id: crate::factions::FactionId },
    CreepStarving { entity: Entity },
}