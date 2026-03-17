use bevy::prelude::*;

use crate::debug::DebugOverlays;
use crate::world::WorldMap;

pub fn room_overlay_system(
    world_map: Res<WorldMap>,
    overlays: Res<DebugOverlays>,
) {
    if overlays.show_room_borders {
        // Render room borders
        tracing::trace!("Rendering room borders for {} rooms", world_map.room_count());
    }

    if overlays.show_ownership {
        // Render ownership colors
    }

    if overlays.show_mine_richness {
        // Render mine richness indicators
    }
}