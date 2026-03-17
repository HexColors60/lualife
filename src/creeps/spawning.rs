use bevy::prelude::*;

use super::{Creep, CreepBody, CreepBundle};
use crate::factions::FactionId;
use crate::world::WorldPos;

static mut NEXT_CREEP_ID: u32 = 1;

fn next_creep_id() -> u32 {
    unsafe {
        let id = NEXT_CREEP_ID;
        NEXT_CREEP_ID += 1;
        id
    }
}

pub fn spawn_creep(
    commands: &mut Commands,
    faction_id: FactionId,
    position: WorldPos,
    body: CreepBody,
) -> Entity {
    let id = next_creep_id();
    let creep = Creep::new(id, faction_id, position, body);
    let bundle = CreepBundle::new(creep);

    commands.spawn(bundle).id()
}