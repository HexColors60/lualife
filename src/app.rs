use bevy::prelude::*;

use crate::accessibility::AccessibilityPlugin;
use crate::achievements::AchievementsPlugin;
use crate::ai::AdvancedAiPlugin;
use crate::audio::AudioPlugin;
use crate::auth::AuthPlugin;
use crate::buildings::BuildingsPlugin;
use crate::chat::ChatPlugin;
use crate::config::GameConfigPlugin;
use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::diplomacy::DiplomacyPlugin;
use crate::discord::DiscordPlugin;
use crate::events::EventsPlugin;
use crate::events_world::WorldEventsPlugin;
use crate::factions::FactionsPlugin;
use crate::localization::LocalizationPlugin;
use crate::lua::LuaPlugin;
use crate::market::MarketPlugin;
use crate::modding::ModdingPlugin;
use crate::network::NetworkPlugin;
use crate::path::PathPlugin;
use crate::performance::PerformancePlugin;
use crate::quality_of_life::QualityOfLifePlugin;
use crate::render::RenderPlugin;
use crate::reputation::ReputationPlugin;
use crate::research::ResearchPlugin;
use crate::save::SavePlugin;
use crate::sim::SimPlugin;
use crate::sync::SyncPlugin;
use crate::territory::TerritoryPlugin;
use crate::trade::TradePlugin;
use crate::tutorial::TutorialPlugin;
use crate::ui::UiPlugin;
use crate::victory::VictoryPlugin;
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
        ));
        app.add_plugins((
            ResearchPlugin,
            MarketPlugin,
            AudioPlugin,
            DiplomacyPlugin,
            ReputationPlugin,
            TerritoryPlugin,
            TradePlugin,
            VictoryPlugin,
        ));
        app.add_plugins((
            NetworkPlugin,
            AuthPlugin,
            ChatPlugin,
            SyncPlugin,
            AdvancedAiPlugin,
            WorldEventsPlugin,
            AchievementsPlugin,
            ModdingPlugin,
        ));
        app.add_plugins((
            PerformancePlugin,
            AccessibilityPlugin,
            LocalizationPlugin,
            TutorialPlugin,
            QualityOfLifePlugin,
            SimPlugin,
            SavePlugin,
            RenderPlugin,
            UiPlugin,
            DiscordPlugin,
        ));
        app.add_plugins((DebugPlugin, LuaPlugin));
    }
}

pub fn create_app() -> App {
    let mut app = App::new();

    app.add_plugins(GameAppPlugin);

    app
}
