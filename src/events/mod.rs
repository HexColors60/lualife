mod combat_events;
mod debug_events;
mod game_events;
mod script_events;
mod ui_events;

pub use combat_events::*;
pub use debug_events::*;
pub use game_events::*;
pub use script_events::*;
pub use ui_events::*;

use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventLog>();
    }
}