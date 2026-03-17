mod command_apply;
mod phases;

pub use command_apply::*;
pub use phases::*;

use bevy::prelude::*;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandBuffer>()
            .add_systems(Update, (
                movement_phase,
                mining_phase,
                combat_phase,
                upkeep_phase,
                economy_phase,
                death_cleanup_phase,
            ).chain().run_if(resource_exists::<crate::world::WorldMap>));
    }
}