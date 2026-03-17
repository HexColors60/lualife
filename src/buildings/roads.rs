use bevy::prelude::*;

use crate::world::WorldPos;

/// Road component - provides movement speed bonus
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Road {
    pub speed_multiplier: f32,
}

impl Road {
    pub fn new() -> Self {
        Self {
            speed_multiplier: 2.0, // Double speed on roads
        }
    }

    pub fn movement_cost_reduction() -> f32 {
        0.5 // Roads reduce movement cost by 50%
    }
}

/// System to apply road speed bonus to creeps
pub fn road_speed_system(
    _creeps: Query<(&crate::creeps::Creep, &mut crate::creeps::CurrentAction)>,
    _roads: Query<&Road, (With<Road>, With<Transform>)>,
    _world_map: Res<crate::world::WorldMap>,
) {
    // This system would check if a creep is on a road and apply speed bonus
    // For now, the movement system handles this via terrain cost
}

/// Check if a position has a road
pub fn has_road_at(_roads: &Query<&Road, (With<Road>, With<Transform>)>, _pos: WorldPos) -> bool {
    // Check if any road is at the given position
    false
}
