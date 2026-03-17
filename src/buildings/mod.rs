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
        app.add_event::<BuildingEvent>();
    }
}

#[derive(Event, Debug, Clone)]
pub enum BuildingEvent {
    BuildingPlaced { entity: Entity, building_type: BuildingType },
    BuildingCompleted { entity: Entity },
    BuildingDestroyed { entity: Entity },
}