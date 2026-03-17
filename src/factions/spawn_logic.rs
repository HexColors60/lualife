use bevy::prelude::*;

use super::{Faction, FactionId, FactionRegistry};
use crate::creeps::{Creep, CreepBundle, CreepBody, spawn_creep};
use crate::resources::Stockpile;
use crate::world::WorldPos;

pub struct FactionSpawnLogic;

impl FactionSpawnLogic {
    pub fn spawn_initial_creep(
        commands: &mut Commands,
        faction_id: FactionId,
        position: WorldPos,
    ) -> Entity {
        let body = CreepBody::default_harvester();
        spawn_creep(commands, faction_id, position, body)
    }

    pub fn spawn_initial_resources(
        stockpile: &mut Stockpile,
    ) {
        // Give each faction some starting resources
        stockpile.add(crate::resources::ResourceType::Power, 1000);
        stockpile.add(crate::resources::ResourceType::Iron, 500);
        stockpile.add(crate::resources::ResourceType::Stone, 300);
    }
}