use bevy::prelude::*;

use crate::world::WorldMap;

pub fn map_render_system(
    world_map: Res<WorldMap>,
) {
    // Placeholder for map rendering
    // In a full implementation, this would render tiles using sprites or a tilemap
    if world_map.room_count() > 0 {
        tracing::trace!("Map has {} rooms", world_map.room_count());
    }
}