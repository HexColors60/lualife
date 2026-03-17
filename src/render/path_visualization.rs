use bevy::prelude::*;

use crate::creeps::Creep;
use crate::debug::DebugOverlays;

/// Marker component for path visualization lines
#[derive(Component)]
pub struct PathVisualization;

/// System to visualize creep paths in debug mode using gizmos
pub fn path_visualization_system(
    creeps: Query<&Creep>,
    debug_overlays: Res<DebugOverlays>,
    mut gizmos: Gizmos,
) {
    // Only show paths if debug overlay is enabled
    if !debug_overlays.show_paths {
        return;
    }

    // Draw paths for creeps with active movement
    for creep in creeps.iter() {
        if let Some(ref action) = creep.current_action {
            if let crate::creeps::CreepAction::MoveTo { target } = action.action {
                // Draw a line from creep position to target
                let start = Vec3::new(
                    creep.position.x as f32 - 128.0,
                    creep.position.y as f32 - 128.0,
                    5.0,
                );
                let end = Vec3::new(
                    target.x as f32 - 128.0,
                    target.y as f32 - 128.0,
                    5.0,
                );

                // Draw path line
                gizmos.line(start, end, Color::srgba(0.0, 1.0, 0.0, 0.8));

                // Draw target marker
                gizmos.circle_2d(Vec2::new(end.x, end.y), 0.5, Color::srgba(1.0, 1.0, 0.0, 0.8));
            }
        }
    }
}

/// System to toggle path visualization
pub fn toggle_path_visualization(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_overlays: ResMut<DebugOverlays>,
) {
    if keyboard.just_pressed(KeyCode::F6) {
        debug_overlays.show_paths = !debug_overlays.show_paths;
    }
}