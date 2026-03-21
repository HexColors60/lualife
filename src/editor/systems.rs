//! Editor systems for modifying the world.
//!
//! Handles terrain painting, mine placement, building placement.

use bevy::prelude::*;

use crate::world::{WorldMap, TerrainType};
use crate::core::TickNumber;

use super::{EditorMode, EditorState, EditorTool, EditorAction, EditorActionEvent};

/// System to handle mouse interactions in editor mode
pub fn editor_paint_system(
    mut state: ResMut<EditorState>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::render::MainCamera>>,
    mut world_map: ResMut<WorldMap>,
    mut action_events: EventWriter<EditorActionEvent>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if !state.enabled || state.mode == EditorMode::None {
        return;
    }

    // Get mouse world position
    let window = windows.iter().next();
    let camera = camera.iter().next();

    if let (Some(window), Some((camera, transform))) = (window, camera) {
        if let Some(cursor) = window.cursor_position() {
            // Convert to world coordinates
            let world_pos = camera.viewport_to_world_2d(transform, cursor);
            if let Some(pos) = world_pos {
                let tile_x = (pos.x + 128.0) as u32;
                let tile_y = (pos.y + 128.0) as u32;

                // Clamp to world bounds
                if tile_x < 256 && tile_y < 256 {
                    state.cursor_position = Some(crate::world::WorldPos::new(tile_x as i32, tile_y as i32));

                    // Paint on left click
                    if mouse.pressed(MouseButton::Left) && state.tool == EditorTool::Paint {
                        apply_brush(
                            &mut state,
                            tile_x,
                            tile_y,
                            &mut world_map,
                            &mut action_events,
                            &mut game_log,
                        );
                    }

                    // Erase on right click
                    if mouse.pressed(MouseButton::Right) && state.tool == EditorTool::Erase {
                        erase_at(
                            &mut state,
                            tile_x,
                            tile_y,
                            &mut world_map,
                            &mut action_events,
                            &mut game_log,
                        );
                    }

                    // Fill on middle click or Shift+Left
                    if (mouse.pressed(MouseButton::Middle) ||
                        (mouse.pressed(MouseButton::Left) && keyboard.pressed(KeyCode::ShiftLeft)))
                        && state.tool == EditorTool::Fill {
                        fill_area(
                            &mut state,
                            tile_x,
                            tile_y,
                            &mut world_map,
                            &mut action_events,
                            &mut game_log,
                        );
                    }
                }
            }
        }
    }
}

/// Apply brush at position
fn apply_brush(
    state: &mut EditorState,
    center_x: u32,
    center_y: u32,
    world_map: &mut WorldMap,
    action_events: &mut EventWriter<EditorActionEvent>,
    game_log: &mut ResMut<crate::ui::GameLog>,
) {
    let radius = state.brush_size.radius();

    match state.mode {
        EditorMode::Terrain => {
            let mut changes = Vec::new();
            let new_terrain = state.selected_terrain;

            for dx in -(radius as i32)..=(radius as i32) {
                for dy in -(radius as i32)..=(radius as i32) {
                    let x = center_x as i32 + dx;
                    let y = center_y as i32 + dy;

                    if x >= 0 && x < 256 && y >= 0 && y < 256 {
                        let room_x = x as u32 / 8;
                        let room_y = y as u32 / 8;
                        let tile_x = x as u32 % 8;
                        let tile_y = y as u32 % 8;

                        if let Some(room) = world_map.get_room_mut(crate::world::RoomCoord::new(room_x, room_y)) {
                            let old_terrain = room.tiles[tile_y as usize][tile_x as usize].terrain;
                            room.tiles[tile_y as usize][tile_x as usize].terrain = new_terrain;
                            changes.push((x as u32, y as u32, new_terrain, old_terrain));
                        }
                    }
                }
            }

            if !changes.is_empty() {
                action_events.send(EditorActionEvent {
                    action: EditorAction::TerrainChange { positions: changes },
                });
            }
        }
        EditorMode::Mines => {
            // Place mine at center
            game_log.add(format!(
                "Place mine {:?} at ({}, {})",
                state.selected_mine_type.resource_type, center_x, center_y
            ));
        }
        EditorMode::Buildings => {
            game_log.add(format!(
                "Place building {:?} at ({}, {})",
                state.selected_building_type, center_x, center_y
            ));
        }
        EditorMode::Factions => {
            game_log.add(format!(
                "Set faction {} spawn at ({}, {})",
                state.selected_faction.0, center_x, center_y
            ));
        }
        EditorMode::Resources => {
            game_log.add(format!(
                "Place resource {:?} at ({}, {})",
                state.selected_resource, center_x, center_y
            ));
        }
        EditorMode::None => {}
    }
}

/// Erase at position
fn erase_at(
    state: &mut EditorState,
    x: u32,
    y: u32,
    world_map: &mut WorldMap,
    action_events: &mut EventWriter<EditorActionEvent>,
    game_log: &mut ResMut<crate::ui::GameLog>,
) {
    match state.mode {
        EditorMode::Terrain => {
            // Reset to plains
            let room_x = x / 8;
            let room_y = y / 8;
            let tile_x = x % 8;
            let tile_y = y % 8;

            if let Some(room) = world_map.get_room_mut(crate::world::RoomCoord::new(room_x, room_y)) {
                let old_terrain = room.tiles[tile_y as usize][tile_x as usize].terrain;
                room.tiles[tile_y as usize][tile_x as usize].terrain = TerrainType::Plains;

                action_events.send(EditorActionEvent {
                    action: EditorAction::TerrainChange {
                        positions: vec![(x, y, TerrainType::Plains, old_terrain)],
                    },
                });
            }
        }
        EditorMode::Mines => {
            game_log.add(format!("Remove mine at ({}, {})", x, y));
        }
        EditorMode::Buildings => {
            game_log.add(format!("Remove building at ({}, {})", x, y));
        }
        _ => {}
    }
}

/// Fill connected area
fn fill_area(
    state: &mut EditorState,
    start_x: u32,
    start_y: u32,
    world_map: &mut WorldMap,
    action_events: &mut EventWriter<EditorActionEvent>,
    game_log: &mut ResMut<crate::ui::GameLog>,
) {
    if state.mode != EditorMode::Terrain {
        return;
    }

    let room_x = start_x / 8;
    let room_y = start_y / 8;
    let tile_x = start_x % 8;
    let tile_y = start_y % 8;

    if let Some(room) = world_map.get_room(crate::world::RoomCoord::new(room_x, room_y)) {
        let target_terrain = room.tiles[tile_y as usize][tile_x as usize].terrain;
        let new_terrain = state.selected_terrain;

        if target_terrain == new_terrain {
            return; // Nothing to fill
        }

        // Simple flood fill (limited for safety)
        let mut changes = Vec::new();
        let mut stack = vec![(start_x, start_y)];
        let mut visited = std::collections::HashSet::new();
        let max_fill = 1000; // Safety limit

        while let Some((x, y)) = stack.pop() {
            if visited.len() >= max_fill {
                break;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            if x >= 256 || y >= 256 {
                continue;
            }

            let rx = x / 8;
            let ry = y / 8;
            let tx = x % 8;
            let ty = y % 8;

            if let Some(room) = world_map.get_room(crate::world::RoomCoord::new(rx, ry)) {
                if room.tiles[ty as usize][tx as usize].terrain != target_terrain {
                    continue;
                }
            }

            visited.insert((x, y));

            // Set new terrain
            if let Some(room) = world_map.get_room_mut(crate::world::RoomCoord::new(rx, ry)) {
                room.tiles[ty as usize][tx as usize].terrain = new_terrain;
                changes.push((x, y, new_terrain, target_terrain));
            }

            // Add neighbors
            if x > 0 { stack.push((x - 1, y)); }
            if x < 255 { stack.push((x + 1, y)); }
            if y > 0 { stack.push((x, y - 1)); }
            if y < 255 { stack.push((x, y + 1)); }
        }

        if !changes.is_empty() {
            action_events.send(EditorActionEvent {
                action: EditorAction::TerrainChange { positions: changes },
            });
            game_log.add(format!("Filled {} tiles", visited.len()));
        }
    }
}

/// System to handle undo/redo
pub fn editor_undo_redo_system(
    mut state: ResMut<EditorState>,
    mut events: EventReader<EditorActionEvent>,
) {
    // Process action events (for recording)
    for event in events.read() {
        state.add_action(event.action.clone());
    }
}

/// Plugin for editor systems
pub struct EditorSystemsPlugin;

impl Plugin for EditorSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                editor_paint_system,
                editor_undo_redo_system,
            ),
        );
    }
}