use bevy::prelude::*;

use super::{Creep, CreepBody, CreepRole};
use crate::factions::FactionId;
use crate::world::WorldPos;

#[derive(Bundle)]
pub struct CreepBundle {
    pub creep: Creep,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: InheritedVisibility,
}

impl CreepBundle {
    pub fn new(creep: Creep) -> Self {
        let x = creep.position.x as f32;
        let y = creep.position.y as f32;

        Self {
            creep,
            transform: Transform::from_xyz(x, y, 1.0),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            computed_visibility: InheritedVisibility::default(),
        }
    }
}
