use bevy::prelude::*;

use crate::config::GameConfigPlugin;
use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::events::EventsPlugin;
use crate::render::RenderPlugin;
use crate::sim::SimPlugin;
use crate::ui::UiPlugin;
use crate::world::WorldPlugin;
use crate::worldgen::WorldgenPlugin;

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameConfigPlugin,
            CorePlugin,
            EventsPlugin,
            WorldPlugin,
            WorldgenPlugin,
            SimPlugin,
            RenderPlugin,
            UiPlugin,
            DebugPlugin,
        ));
    }
}

pub fn create_app() -> App {
    let mut app = App::new();

    app.add_plugins(GameAppPlugin);

    app
}