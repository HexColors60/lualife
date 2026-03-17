mod building_type;
mod bundle;
mod construction;
mod defense;
mod production;
mod refinery;
mod repair;
mod roads;
mod spawn;
mod storage;

pub use building_type::*;
pub use bundle::*;
pub use construction::*;
pub use defense::*;
pub use production::*;
pub use refinery::*;
pub use repair::*;
pub use roads::*;
pub use spawn::*;
pub use storage::*;

use bevy::prelude::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingIdGenerator>()
            .add_event::<BuildingEvent>()
            .add_systems(Update, (
                spawn_initial_buildings.run_if(resource_exists::<crate::factions::FactionRegistry>),
                tower_attack_system,
            ));
    }
}

#[derive(Resource, Default)]
pub struct BuildingIdGenerator {
    next_id: u32,
}

impl BuildingIdGenerator {
    pub fn next(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

/// System to spawn initial buildings for each faction
fn spawn_initial_buildings(
    mut commands: Commands,
    mut id_gen: ResMut<BuildingIdGenerator>,
    factions: Res<crate::factions::FactionRegistry>,
    mut game_log: ResMut<crate::ui::GameLog>,
    existing_buildings: Query<&Building>,
) {
    // Only spawn once - check if any buildings exist
    if !existing_buildings.is_empty() {
        return;
    }

    let mut count = 0;
    for faction in factions.all() {
        // Spawn a BaseCore for each faction at a random position near their spawn
        let base_x = (faction.id.0 as i32 % 32) * 8 + 4;
        let base_y = (faction.id.0 as i32 / 32) * 8 + 4;

        let building = Building::new(
            id_gen.next(),
            BuildingType::BaseCore,
            faction.id,
            crate::world::WorldPos::new(base_x, base_y),
        );

        commands.spawn(building);
        count += 1;
    }

    if count > 0 {
        game_log.add(format!("Spawned {} base cores for factions", count));
    }
}

#[derive(Event, Debug, Clone)]
pub enum BuildingEvent {
    BuildingPlaced { entity: Entity, building_type: BuildingType },
    BuildingCompleted { entity: Entity },
    BuildingDestroyed { entity: Entity },
}