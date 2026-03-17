mod dropped_resource;
mod economy;
mod recipes;
mod resource_type;
mod stockpile;

pub use dropped_resource::*;
pub use economy::*;
pub use recipes::*;
pub use resource_type::*;
pub use stockpile::*;

use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResourceEvent>();
    }
}

#[derive(Event, Debug, Clone)]
pub enum ResourceEvent {
    ResourceDropped { x: i32, y: i32, resource_type: ResourceType, amount: u32 },
    ResourcePickedUp { entity: Entity, resource_type: ResourceType, amount: u32 },
    ResourceTransferred { from: Entity, to: Entity, resource_type: ResourceType, amount: u32 },
}