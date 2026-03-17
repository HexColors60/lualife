mod actions;
mod autonomous_ai;
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
pub use autonomous_ai::*;
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
        app.init_resource::<CreepIdGenerator>()
            .add_event::<CreepEvent>()
            .add_systems(
                Update,
                (
                    spawn_initial_creeps.run_if(resource_exists::<crate::world::WorldMap>),
                    autonomous_creep_ai,
                ),
            );
    }
}

#[derive(Event, Debug, Clone)]
pub enum CreepEvent {
    CreepSpawned {
        entity: Entity,
        faction_id: crate::factions::FactionId,
    },
    CreepDied {
        entity: Entity,
        faction_id: crate::factions::FactionId,
    },
    CreepStarving {
        entity: Entity,
    },
}
