use bevy::prelude::*;

use crate::buildings::Tower;
use crate::debug::DebugOverlays;

/// System to visualize tower attack ranges in debug mode
pub fn tower_range_visualization_system(
    towers: Query<(&Tower, &Transform)>,
    debug_overlays: Res<DebugOverlays>,
    mut gizmos: Gizmos,
) {
    // Only show ranges if debug overlay is enabled
    if !debug_overlays.show_danger_zones {
        return;
    }

    // Draw attack range for each tower
    for (tower, transform) in towers.iter() {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Draw attack range circle
        gizmos.circle_2d(pos, tower.range, Color::srgba(1.0, 0.0, 0.0, 0.3));

        // Draw inner effective range
        gizmos.circle_2d(pos, tower.range * 0.5, Color::srgba(1.0, 0.5, 0.0, 0.2));

        // Draw tower center
        gizmos.circle_2d(pos, 0.3, Color::srgba(1.0, 0.0, 0.0, 0.8));
    }
}

/// System to toggle tower range visualization
pub fn toggle_tower_range_visualization(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_overlays: ResMut<DebugOverlays>,
) {
    if keyboard.just_pressed(KeyCode::F7) {
        debug_overlays.show_danger_zones = !debug_overlays.show_danger_zones;
    }
}
