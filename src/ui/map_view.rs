use bevy::prelude::*;

use crate::config::UiConfig;
use crate::debug::SelectionState;
use crate::world::WorldMap;

pub fn map_view_system(
    _selection: ResMut<SelectionState>,
    _world_map: Res<WorldMap>,
    camera: Query<&Transform, With<crate::render::MainCamera>>,
    windows: Query<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    _config: Res<UiConfig>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert screen position to world position
                if let Ok(camera_transform) = camera.get_single() {
                    let world_pos = screen_to_world(
                        cursor_pos,
                        window.width(),
                        window.height(),
                        camera_transform,
                    );

                    // Check if there's an entity at this position
                    // For now, just log the click
                    tracing::debug!("Map clicked at world position: {:?}", world_pos);
                }
            }
        }
    }
}

fn screen_to_world(
    screen_pos: Vec2,
    screen_width: f32,
    screen_height: f32,
    camera_transform: &Transform,
) -> Vec2 {
    let ndc_x = (screen_pos.x / screen_width - 0.5) * 2.0;
    let ndc_y = (screen_pos.y / screen_height - 0.5) * 2.0;

    let world_pos = camera_transform.compute_matrix() * Vec4::new(ndc_x, -ndc_y, 0.0, 1.0);

    Vec2::new(world_pos.x, world_pos.y)
}