use bevy::prelude::*;
use std::collections::HashMap;

use crate::factions::FactionId;
use crate::resources::ResourceType;
use crate::world::WorldPos;

/// A trade route between two locations
#[derive(Debug, Clone, Component)]
pub struct TradeRoute {
    pub id: u32,
    pub owner_faction: FactionId,
    pub start_pos: WorldPos,
    pub end_pos: WorldPos,
    pub resource_type: ResourceType,
    pub quantity_per_trip: u32,
    pub travel_time: u32, // ticks
    pub active: bool,
}

impl TradeRoute {
    pub fn new(
        id: u32,
        owner_faction: FactionId,
        start_pos: WorldPos,
        end_pos: WorldPos,
        resource_type: ResourceType,
        quantity_per_trip: u32,
    ) -> Self {
        // Calculate travel time based on distance
        let dx = (end_pos.x - start_pos.x).abs();
        let dy = (end_pos.y - start_pos.y).abs();
        let distance = ((dx + dy) as f32 * 1.5) as u32; // Manhattan distance with factor

        Self {
            id,
            owner_faction,
            start_pos,
            end_pos,
            resource_type,
            quantity_per_trip,
            travel_time: distance.max(50), // Minimum 50 ticks
            active: true,
        }
    }
}

/// A caravan traveling along a trade route
#[derive(Debug, Clone, Component)]
pub struct Caravan {
    pub id: u32,
    pub route_id: u32,
    pub faction_id: FactionId,
    pub resource_type: ResourceType,
    pub quantity: u32,
    pub position: WorldPos,
    pub target: WorldPos,
    pub progress: u32,   // ticks traveled
    pub total_time: u32, // total travel time
    pub returning: bool, // true if returning to start
}

impl Caravan {
    pub fn new(
        id: u32,
        route_id: u32,
        faction_id: FactionId,
        start: WorldPos,
        end: WorldPos,
        resource_type: ResourceType,
        quantity: u32,
        travel_time: u32,
    ) -> Self {
        Self {
            id,
            route_id,
            faction_id,
            resource_type,
            quantity,
            position: start,
            target: end,
            progress: 0,
            total_time: travel_time,
            returning: false,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.progress += 1;
        self.progress >= self.total_time
    }

    pub fn arrive(&mut self) {
        // Swap position and target
        std::mem::swap(&mut self.position, &mut self.target);
        self.progress = 0;
        self.returning = !self.returning;
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.total_time > 0 {
            self.progress as f32 / self.total_time as f32
        } else {
            0.0
        }
    }
}

/// Trade route manager
#[derive(Resource, Debug, Clone, Default)]
pub struct TradeRouteManager {
    pub routes: HashMap<u32, TradeRoute>,
    pub next_route_id: u32,
    pub next_caravan_id: u32,
}

impl TradeRouteManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_route(
        &mut self,
        owner_faction: FactionId,
        start_pos: WorldPos,
        end_pos: WorldPos,
        resource_type: ResourceType,
        quantity_per_trip: u32,
    ) -> u32 {
        let id = self.next_route_id;
        self.next_route_id += 1;

        let route = TradeRoute::new(
            id,
            owner_faction,
            start_pos,
            end_pos,
            resource_type,
            quantity_per_trip,
        );

        self.routes.insert(id, route);
        id
    }

    pub fn spawn_caravan(&mut self, route_id: u32) -> Option<Caravan> {
        let route = self.routes.get(&route_id)?;
        if !route.active {
            return None;
        }

        let caravan_id = self.next_caravan_id;
        self.next_caravan_id += 1;

        Some(Caravan::new(
            caravan_id,
            route_id,
            route.owner_faction,
            route.start_pos,
            route.end_pos,
            route.resource_type,
            route.quantity_per_trip,
            route.travel_time,
        ))
    }
}

/// System to update caravans
pub fn caravan_movement_system(
    mut caravans: Query<&mut Caravan>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for mut caravan in caravans.iter_mut() {
        if caravan.tick() {
            caravan.arrive();

            // Log arrival
            let direction = if caravan.returning {
                "returning"
            } else {
                "delivering"
            };

            game_log.add(format!(
                "Caravan {} {} {} {} {}",
                caravan.id,
                direction,
                caravan.quantity,
                caravan.resource_type.name(),
                if caravan.returning {
                    "home"
                } else {
                    "to destination"
                }
            ));
        }
    }
}

/// Plugin for trade system
pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TradeRouteManager>()
            .add_systems(Update, caravan_movement_system);
    }
}
