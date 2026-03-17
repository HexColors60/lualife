use bevy::prelude::*;

use crate::buildings::BuildingsPlugin;
use crate::config::GameConfigPlugin;
use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::events::EventsPlugin;
use crate::factions::FactionsPlugin;
use crate::market::MarketPlugin;
use crate::path::PathPlugin;
use crate::render::RenderPlugin;
use crate::research::ResearchPlugin;
use crate::save::SavePlugin;
use crate::sim::SimPlugin;
use crate::ui::UiPlugin;
use crate::world::WorldPlugin;
use crate::worldgen::WorldgenPlugin;

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy default plugins first (includes Time, Window, etc.)
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_screeps_lua".to_string(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }));

        // Set clear color (dark background)
        app.insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)));

        // Add game plugins
        app.add_plugins((
            GameConfigPlugin,
            CorePlugin,
            EventsPlugin,
            FactionsPlugin,
            WorldPlugin,
            WorldgenPlugin,
            BuildingsPlugin,
            PathPlugin,
            ResearchPlugin,
            MarketPlugin,
            SimPlugin,
            SavePlugin,
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