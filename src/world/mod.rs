mod coordinates;
mod ownership;
mod room;
mod room_index;
mod terrain;
mod tile;
mod visibility;
mod world_builder;
mod world_map;

pub use coordinates::*;
pub use ownership::*;
pub use room::*;
pub use room_index::*;
pub use terrain::*;
pub use tile::*;
pub use visibility::*;
pub use world_builder::*;
pub use world_map::*;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>();
        app.add_event::<WorldEvent>();
    }
}

#[derive(Event, Debug, Clone)]
pub enum WorldEvent {
    RoomGenerated { room_x: u32, room_y: u32 },
    TileChanged { x: i32, y: i32 },
}
