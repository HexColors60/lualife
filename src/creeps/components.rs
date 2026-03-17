use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{CreepBody, CreepRole, Inventory};
use crate::factions::FactionId;
use crate::world::WorldPos;

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Creep {
    pub id: u32,
    pub faction_id: FactionId,
    pub position: WorldPos,
    pub body: CreepBody,
    pub hp: f32,
    pub max_hp: f32,
    pub power_reserve: f32,
    pub max_power: f32,
    pub inventory: Inventory,
    pub role: CreepRole,
    pub current_action: Option<CurrentAction>,
}

impl Creep {
    pub fn new(id: u32, faction_id: FactionId, position: WorldPos, body: CreepBody) -> Self {
        let max_hp = body.max_hp();
        let max_power = body.max_power();
        let carry_capacity = body.carry_capacity();

        Self {
            id,
            faction_id,
            position,
            body,
            hp: max_hp,
            max_hp,
            power_reserve: max_power,
            max_power,
            inventory: Inventory::new(carry_capacity),
            role: CreepRole::default(),
            current_action: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn is_starving(&self) -> bool {
        self.power_reserve < self.max_power * 0.2
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn consume_power(&mut self, amount: f32) -> f32 {
        let consumed = amount.min(self.power_reserve);
        self.power_reserve -= consumed;
        consumed
    }

    pub fn recharge_power(&mut self, amount: f32) {
        self.power_reserve = (self.power_reserve + amount).min(self.max_power);
    }
}

#[derive(Debug, Clone, Component, Default)]
pub struct CreepId(pub u32);

#[derive(Debug, Clone, Component)]
pub struct OwnerFaction(pub FactionId);

#[derive(Debug, Clone, Component)]
pub struct HitPoints {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Component)]
pub struct PowerReserve {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct CurrentAction {
    pub action: super::CreepAction,
    pub target_id: Option<u32>,
    pub progress: f32,
}

#[derive(Debug, Clone, Component)]
pub struct VisionRange(pub u32);

impl Default for VisionRange {
    fn default() -> Self {
        Self(5)
    }
}
