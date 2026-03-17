use bevy::prelude::*;

use super::{Creep, CreepAction};
use crate::world::WorldMap;
use crate::world::WorldPos;

pub fn creep_movement_system(mut creeps: Query<&mut Creep>, world_map: Res<WorldMap>) {
    for mut creep in creeps.iter_mut() {
        if let Some(CreepAction::MoveTo { target }) =
            &creep.current_action.as_ref().map(|a| &a.action)
        {
            let target = *target;

            // Calculate direction
            let dx = (target.x - creep.position.x).signum();
            let dy = (target.y - creep.position.y).signum();

            // Calculate new position
            let speed = creep.body.speed();
            if speed <= 0.0 {
                continue;
            }

            let new_x = creep.position.x + dx;
            let new_y = creep.position.y + dy;

            let new_pos = WorldPos::new(new_x, new_y);

            // Check if walkable
            if world_map.is_walkable(new_pos) {
                creep.position = new_pos;
            }

            // Check if reached target
            if creep.position == target {
                creep.current_action = None;
            }
        }
    }
}
