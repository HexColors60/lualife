mod load_game;
mod save_game;
mod snapshot;
mod versioning;

pub use load_game::*;
pub use save_game::*;
pub use snapshot::*;
pub use versioning::*;

use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, _app: &mut App) {
        // Save/load systems registered on demand
    }
}