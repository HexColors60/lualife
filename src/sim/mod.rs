mod command_apply;
mod phases;

pub use command_apply::*;
pub use phases::*;

use bevy::prelude::*;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandBuffer>();
    }
}