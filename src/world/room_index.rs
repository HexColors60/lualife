use std::collections::HashMap;

use bevy::prelude::*;

use crate::world::{RoomCoord, WorldPos};

/// Fast lookup indices for room contents
#[derive(Resource, Debug, Clone, Default)]
pub struct RoomIndex {
    /// Map from entity ID to room coordinate
    pub entity_to_room: HashMap<u32, RoomCoord>,
    /// Map from position to entity IDs at that position
    pub position_entities: HashMap<(i32, i32), Vec<Entity>>,
}

impl RoomIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity(&mut self, entity: Entity, pos: WorldPos, room: RoomCoord) {
        let id = entity.index();
        self.entity_to_room.insert(id, room);
        self.position_entities
            .entry((pos.x, pos.y))
            .or_default()
            .push(entity);
    }

    pub fn unregister_entity(&mut self, entity: Entity, pos: WorldPos) {
        let id = entity.index();
        self.entity_to_room.remove(&id);
        if let Some(entities) = self.position_entities.get_mut(&(pos.x, pos.y)) {
            entities.retain(|&e| e != entity);
            if entities.is_empty() {
                self.position_entities.remove(&(pos.x, pos.y));
            }
        }
    }

    pub fn get_room(&self, entity: Entity) -> Option<RoomCoord> {
        self.entity_to_room.get(&entity.index()).copied()
    }

    pub fn get_entities_at(&self, pos: WorldPos) -> Option<&Vec<Entity>> {
        self.position_entities.get(&(pos.x, pos.y))
    }

    pub fn move_entity(&mut self, entity: Entity, from: WorldPos, to: WorldPos, room: RoomCoord) {
        self.unregister_entity(entity, from);
        self.register_entity(entity, to, room);
    }
}
