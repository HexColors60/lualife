use bevy::prelude::*;

use crate::creeps::Creep;
use crate::debug::{DebugOverlays, GodMode};
use crate::path::PathCache;
use crate::render::MainCamera;
use crate::world::WorldMap;

/// Marker for debug overlay entities
#[derive(Component)]
pub struct DebugOverlay;

/// Marker for grid overlay
#[derive(Component)]
pub struct GridOverlay;

/// Marker for FPS display
#[derive(Component)]
pub struct FpsText;

/// Setup debug overlays
pub fn setup_debug_overlays(mut commands: Commands) {
    // FPS counter at top-right (below unit panel)
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "FPS: 0",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.5, 1.0, 0.5),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(200.0),
                right: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            ..default()
        },
        FpsText,
    ));

    // Entity count display
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Entities: 0",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.7, 0.7, 1.0),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(220.0),
                right: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            ..default()
        },
        DebugOverlay,
    ));
}

/// Update FPS display
pub fn update_fps_display(
    mut query: Query<&mut Text, With<FpsText>>,
    time: Res<Time>,
    mut frame_count: Local<u32>,
    mut fps_timer: Local<f32>,
    mut last_fps: Local<f32>,
) {
    *frame_count += 1;
    *fps_timer += time.delta_seconds();

    if *fps_timer >= 1.0 {
        *last_fps = *frame_count as f32 / *fps_timer;
        *frame_count = 0;
        *fps_timer = 0.0;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("FPS: {:.0}", *last_fps);
    }
}

/// Update entity count display
pub fn update_entity_count(
    mut query: Query<&mut Text, (With<DebugOverlay>, Without<FpsText>)>,
    creeps: Query<&Creep>,
    mines: Query<&crate::mines::MineNode>,
    buildings: Query<&crate::buildings::Building>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Creeps: {} | Mines: {} | Buildings: {}",
            creeps.iter().count(),
            mines.iter().count(),
            buildings.iter().count()
        );
    }
}

/// Toggle debug overlays with F3
pub fn toggle_debug_overlays(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlays: ResMut<DebugOverlays>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        overlays.show_grid = !overlays.show_grid;
        overlays.show_paths = !overlays.show_paths;
        overlays.show_danger_zones = !overlays.show_danger_zones;
    }
}

pub fn debug_render_system(
    _world_map: Res<WorldMap>,
    god_mode: Res<GodMode>,
    overlays: Res<DebugOverlays>,
    path_cache: Res<PathCache>,
) {
    if !god_mode.enabled {
        return;
    }

    if overlays.show_paths {
        // Render cached paths
        tracing::trace!("Path cache has {} entries", path_cache.len());
    }

    if overlays.show_grid {
        // Render grid overlay
    }

    if overlays.show_danger_zones {
        // Render danger zone indicators
    }
}
