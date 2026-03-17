mod astar;
mod cost_map;
mod flow_field;
mod path_cache;
mod room_graph;

pub use astar::*;
pub use cost_map::*;
pub use flow_field::*;
pub use path_cache::*;
pub use room_graph::*;

use bevy::prelude::*;

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathCache>();
    }
}