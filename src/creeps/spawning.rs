use bevy::prelude::*;

use super::{Creep, CreepBody, CreepBundle, CreepRole};
use crate::factions::FactionId;
use crate::world::WorldPos;

/// Resource for generating unique creep IDs
#[derive(Resource, Default)]
pub struct CreepIdGenerator {
    next_id: u32,
}

impl CreepIdGenerator {
    pub fn next(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub fn spawn_creep(
    commands: &mut Commands,
    id_gen: &mut CreepIdGenerator,
    faction_id: FactionId,
    position: WorldPos,
    body: CreepBody,
) -> Entity {
    let id = id_gen.next();
    let creep = Creep::new(id, faction_id, position, body);
    let bundle = CreepBundle::new(creep);

    commands.spawn(bundle).id()
}

/// System to spawn initial creeps for each faction
pub fn spawn_initial_creeps(
    mut commands: Commands,
    mut id_gen: ResMut<CreepIdGenerator>,
    faction_registry: Res<crate::factions::FactionRegistry>,
    world_map: Res<crate::world::WorldMap>,
    creep_query: Query<&super::Creep>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Only spawn if no creeps exist yet
    if !creep_query.is_empty() {
        return;
    }

    let faction_count = faction_registry.count();
    if faction_count == 0 {
        return;
    }

    // Find valid spawn positions for each faction
    let rooms: Vec<_> = world_map.all_rooms().collect();
    if rooms.is_empty() {
        return;
    }

    game_log.add(format!("Spawning creeps for {} factions...", faction_count));

    // Spawn one harvester creep for each faction
    for (i, faction) in faction_registry.all().enumerate() {
        // Pick a room for this faction (spread across the map)
        let room_index = (i * rooms.len() / faction_count).min(rooms.len() - 1);
        let room = &rooms[room_index];

        // Spawn position in center of room
        let spawn_x = (room.coord.x * crate::consts::ROOM_TILE_SIZE + 4) as i32;
        let spawn_y = (room.coord.y * crate::consts::ROOM_TILE_SIZE + 4) as i32;
        let position = WorldPos::new(spawn_x, spawn_y);

        // Create a basic harvester body
        let body = CreepBody::harvester();

        let _entity = spawn_creep(&mut commands, &mut id_gen, faction.id, position, body);

        game_log.add(format!(
            "Spawned creep for {} at ({}, {})",
            faction.name, spawn_x, spawn_y
        ));
    }
}