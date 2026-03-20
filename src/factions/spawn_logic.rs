use bevy::prelude::*;

use super::FactionId;
use crate::creeps::{spawn_creep, CreepBody, CreepIdGenerator};
use crate::resources::Stockpile;
use crate::world::WorldPos;

pub struct FactionSpawnLogic;

impl FactionSpawnLogic {
    pub fn spawn_initial_creep(
        commands: &mut Commands,
        id_gen: &mut CreepIdGenerator,
        faction_id: FactionId,
        position: WorldPos,
    ) -> Entity {
        let body = CreepBody::default_harvester();
        spawn_creep(commands, id_gen, faction_id, position, body)
    }

    pub fn spawn_initial_resources(stockpile: &mut Stockpile) {
        // Give each faction some starting resources
        stockpile.add(crate::resources::ResourceType::Power, 1000);
        stockpile.add(crate::resources::ResourceType::Iron, 500);
        stockpile.add(crate::resources::ResourceType::Stone, 300);
    }
}
