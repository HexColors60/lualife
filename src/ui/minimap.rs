use bevy::prelude::*;

use crate::consts::{ROOM_GRID_X, ROOM_GRID_Y};
use crate::world::WorldMap;

#[derive(Resource, Debug, Clone)]
pub struct MinimapState {
    pub hovered_room: Option<(u32, u32)>,
}

impl Default for MinimapState {
    fn default() -> Self {
        Self {
            hovered_room: None,
        }
    }
}

pub fn minimap_click_system(
    mut events: EventReader<crate::events::UiEvent>,
    mut camera: Query<&mut Transform, With<crate::render::MainCamera>>,
) {
    for event in events.read() {
        if let crate::events::UiEvent::MinimapClicked { room_x, room_y } = event {
            // Move camera to center of room
            let world_x = (room_x * crate::consts::ROOM_TILE_SIZE + crate::consts::ROOM_TILE_SIZE / 2) as f32;
            let world_y = (room_y * crate::consts::ROOM_TILE_SIZE + crate::consts::ROOM_TILE_SIZE / 2) as f32;

            for mut transform in camera.iter_mut() {
                transform.translation.x = world_x;
                transform.translation.y = world_y;
            }
        }
    }
}