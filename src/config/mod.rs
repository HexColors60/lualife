mod ai_config;
mod game_config;
mod ui_config;
mod worldgen_config;

pub use ai_config::*;
pub use game_config::*;
pub use ui_config::*;
pub use worldgen_config::*;

use bevy::prelude::*;
use std::path::Path;

use crate::consts::*;
use crate::error::GameResult;

pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>()
            .init_resource::<WorldgenConfig>()
            .init_resource::<AiConfig>()
            .init_resource::<UiConfig>();
    }
}

fn load_config_or_default<T>(path: &Path) -> T
where
    T: Default + serde::de::DeserializeOwned,
{
    match std::fs::read_to_string(path) {
        Ok(content) => match ron::from_str(&content) {
            Ok(config) => {
                tracing::info!("Loaded config from {:?}", path);
                config
            }
            Err(e) => {
                tracing::warn!("Failed to parse config {:?}: {}, using defaults", path, e);
                T::default()
            }
        },
        Err(e) => {
            tracing::info!(
                "Config file {:?} not found ({}), using defaults",
                path,
                e
            );
            T::default()
        }
    }
}