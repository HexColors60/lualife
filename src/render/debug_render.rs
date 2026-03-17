use bevy::prelude::*;

use crate::debug::{DebugOverlays, GodMode};
use crate::path::PathCache;
use crate::world::WorldMap;

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