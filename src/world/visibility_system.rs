//! Visibility and fog of war system.
//!
//! Manages what each faction can see based on unit positions.

use bevy::prelude::*;

use crate::consts::ROOM_TILE_SIZE;
use crate::creeps::Creep;
use crate::factions::FactionId;
use crate::world::{RoomCoord, WorldMap};

/// Plugin for visibility systems.
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_visibility_from_creeps);
    }
}

/// Vision range in tiles for a creep (how far they can see).
const CREEP_VISION_RANGE: i32 = 10;

/// Update room visibility based on creep positions.
/// Each creep reveals rooms within its vision range.
fn update_visibility_from_creeps(
    creeps: Query<&Creep>,
    mut world_map: ResMut<WorldMap>,
    god_mode: Res<crate::debug::GodMode>,
) {
    // In GOD mode, all rooms are visible
    if god_mode.enabled {
        for room in world_map.all_rooms_mut() {
            room.visibility.discovered = true;
        }
        return;
    }

    // Clear previous visibility for all factions
    for room in world_map.all_rooms_mut() {
        room.visibility.clear_visibility();
    }

    // Collect creep positions by faction
    let mut faction_positions: std::collections::HashMap<FactionId, Vec<(i32, i32)>> =
        std::collections::HashMap::new();

    for creep in creeps.iter() {
        faction_positions
            .entry(creep.faction_id)
            .or_insert_with(Vec::new)
            .push((creep.position.x, creep.position.y));
    }

    // Update visibility for each faction based on their creeps
    for (faction_id, positions) in faction_positions {
        for (x, y) in positions {
            // Calculate which rooms are visible from this position
            let room_x = x / ROOM_TILE_SIZE as i32;
            let room_y = y / ROOM_TILE_SIZE as i32;

            // Reveal rooms within vision range
            let rooms_in_range = (CREEP_VISION_RANGE / ROOM_TILE_SIZE as i32).max(1);

            for dx in -rooms_in_range..=rooms_in_range {
                for dy in -rooms_in_range..=rooms_in_range {
                    if let Some(coord) = RoomCoord::try_new(room_x + dx, room_y + dy) {
                        if let Some(room) = world_map.get_room_mut(coord) {
                            room.visibility.set_visible(faction_id);
                        }
                    }
                }
            }
        }
    }
}

/// Check if a room is visible to a specific faction.
pub fn is_room_visible_to(world_map: &WorldMap, coord: RoomCoord, faction_id: FactionId) -> bool {
    world_map
        .get_room(coord)
        .map(|room| room.visibility.is_visible_to(faction_id))
        .unwrap_or(false)
}

/// Check if a room has been discovered by any faction.
pub fn is_room_discovered(world_map: &WorldMap, coord: RoomCoord) -> bool {
    world_map
        .get_room(coord)
        .map(|room| room.visibility.discovered)
        .unwrap_or(false)
}

/// Get fog of war alpha for rendering (0.0 = fully visible, 1.0 = completely hidden).
pub fn get_fog_alpha(
    world_map: &WorldMap,
    coord: RoomCoord,
    viewer_faction: Option<FactionId>,
    god_mode: bool,
) -> f32 {
    if god_mode {
        return 0.0; // No fog in GOD mode
    }

    let room = match world_map.get_room(coord) {
        Some(r) => r,
        None => return 1.0, // Unknown room = full fog
    };

    // Check if any faction is viewing
    if let Some(faction_id) = viewer_faction {
        if room.visibility.is_visible_to(faction_id) {
            return 0.0; // Currently visible = no fog
        }
    }

    // Previously discovered but not currently visible
    if room.visibility.discovered {
        return 0.5; // Partial fog (dimmed)
    }

    // Never discovered
    1.0 // Full fog
}