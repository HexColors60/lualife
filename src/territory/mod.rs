use std::collections::HashMap;
use bevy::prelude::*;

use crate::factions::FactionId;
use crate::world::RoomCoord;

/// Territory ownership for a room
#[derive(Debug, Clone, Component)]
pub struct Territory {
    pub room_coord: RoomCoord,
    pub owner: Option<FactionId>,
    pub claim_strength: u32,
    pub contesting_factions: Vec<FactionId>,
}

impl Territory {
    pub fn new(room_coord: RoomCoord) -> Self {
        Self {
            room_coord,
            owner: None,
            claim_strength: 0,
            contesting_factions: Vec::new(),
        }
    }

    pub fn is_claimed(&self) -> bool {
        self.owner.is_some()
    }

    pub fn claim(&mut self, faction: FactionId, strength: u32) {
        self.owner = Some(faction);
        self.claim_strength = strength;
        self.contesting_factions.clear();
    }

    pub fn contest(&mut self, faction: FactionId) {
        if !self.contesting_factions.contains(&faction) {
            self.contesting_factions.push(faction);
        }
    }

    pub fn unclaim(&mut self) {
        self.owner = None;
        self.claim_strength = 0;
    }
}

/// Territory control manager
#[derive(Resource, Debug, Clone, Default)]
pub struct TerritoryManager {
    pub territories: HashMap<RoomCoord, Territory>,
}

impl TerritoryManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_territory(&self, coord: RoomCoord) -> Option<&Territory> {
        self.territories.get(&coord)
    }

    pub fn get_territory_mut(&mut self, coord: RoomCoord) -> Option<&mut Territory> {
        self.territories.get_mut(&coord)
    }

    pub fn claim_room(&mut self, coord: RoomCoord, faction: FactionId, strength: u32) {
        if let Some(territory) = self.territories.get_mut(&coord) {
            territory.claim(faction, strength);
        } else {
            let mut territory = Territory::new(coord);
            territory.claim(faction, strength);
            self.territories.insert(coord, territory);
        }
    }

    pub fn get_faction_territories(&self, faction: FactionId) -> Vec<RoomCoord> {
        self.territories
            .values()
            .filter(|t| t.owner == Some(faction))
            .map(|t| t.room_coord)
            .collect()
    }

    pub fn get_territory_count(&self, faction: FactionId) -> usize {
        self.territories
            .values()
            .filter(|t| t.owner == Some(faction))
            .count()
    }

    pub fn total_claimed(&self) -> usize {
        self.territories
            .values()
            .filter(|t| t.is_claimed())
            .count()
    }
}

/// System to process territory claims
pub fn territory_claim_system(
    mut territory_manager: ResMut<TerritoryManager>,
    buildings: Query<&crate::buildings::Building>,
    world_map: Res<crate::world::WorldMap>,
) {
    // Buildings with BaseCore type claim territory around them
    for building in buildings.iter() {
        if building.building_type == crate::buildings::BuildingType::BaseCore {
            // Calculate room coord from position
            let room_x = (building.position.x / 8) as u32;
            let room_y = (building.position.y / 8) as u32;
            let room_coord = crate::world::RoomCoord::new(room_x, room_y);

            if world_map.get_room(room_coord).is_some() {
                territory_manager.claim_room(
                    room_coord,
                    building.faction_id,
                    100, // Base claim strength
                );
            }
        }
    }
}

/// Plugin for territory system
pub struct TerritoryPlugin;

impl Plugin for TerritoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerritoryManager>()
            .add_systems(Update, territory_claim_system);
    }
}