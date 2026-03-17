mod mine_node;
mod mine_type;
mod mining_logic;
mod regeneration;

pub use mine_node::*;
pub use mine_type::*;
pub use mining_logic::*;
pub use regeneration::*;

use bevy::prelude::*;

pub struct MinesPlugin;

impl Plugin for MinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MineEvent>();
    }
}

#[derive(Event, Debug, Clone)]
pub enum MineEvent {
    MineExhausted { mine_id: u32 },
    MineRegenerated { mine_id: u32 },
    MineDepleted { mine_id: u32, amount: u32 },
}