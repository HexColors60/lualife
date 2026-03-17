use bevy::prelude::*;

use super::BuildingType;
use crate::factions::FactionId;
use crate::world::WorldPos;

#[derive(Debug, Clone, Component)]
pub struct Building {
    pub id: u32,
    pub building_type: BuildingType,
    pub faction_id: FactionId,
    pub position: WorldPos,
    pub hp: f32,
    pub max_hp: f32,
    pub level: u8,
    pub active: bool,
}

impl Building {
    pub fn new(id: u32, building_type: BuildingType, faction_id: FactionId, position: WorldPos) -> Self {
        let max_hp = 1000.0; // Default HP

        Self {
            id,
            building_type,
            faction_id,
            position,
            hp: max_hp,
            max_hp,
            level: 1,
            active: true,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    pub fn repair(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn is_destroyed(&self) -> bool {
        self.hp <= 0.0
    }
}

#[derive(Bundle)]
pub struct BuildingBundle {
    pub building: Building,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}

impl BuildingBundle {
    pub fn new(building: Building) -> Self {
        let x = building.position.x as f32;
        let y = building.position.y as f32;

        Self {
            building,
            transform: Transform::from_xyz(x, y, 0.5),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
        }
    }
}