mod layout;
mod log_panel;
mod map_view;
mod minimap;
mod perf_panel;
mod selection;
mod unit_panel;
mod widgets;

pub use layout::*;
pub use log_panel::*;
pub use map_view::*;
pub use minimap::*;
pub use perf_panel::*;
pub use selection::*;
pub use unit_panel::*;
pub use widgets::*;

use bevy::prelude::*;

use crate::config::UiConfig;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>();

        app.add_systems(Startup, setup_ui);
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct UiState {
    pub show_minimap: bool,
    pub show_log_panel: bool,
    pub show_perf_panel: bool,
    pub show_unit_panel: bool,
}

fn setup_ui(mut commands: Commands, config: Res<UiConfig>) {
    commands.insert_resource(UiState {
        show_minimap: config.show_minimap,
        show_log_panel: config.show_log_panel,
        show_perf_panel: config.show_perf_panel,
        show_unit_panel: false,
    });
}