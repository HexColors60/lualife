mod camera;
mod creep_render;
mod debug_render;
mod healthbars;
mod map_render;
mod mine_render;
mod room_overlay;

pub use camera::*;
pub use creep_render::*;
pub use debug_render::*;
pub use healthbars::*;
pub use map_render::*;
pub use mine_render::*;
pub use room_overlay::*;

use bevy::prelude::*;

use crate::config::UiConfig;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_render);
    }
}

fn setup_render(mut commands: Commands) {
    // Main camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(128.0, 128.0, 1000.0),
            ..default()
        },
        MainCamera,
    ));
}